use anyhow::Context;
use aws_config::{
    environment::EnvironmentVariableCredentialsProvider,
    imds::credentials::ImdsCredentialsProvider,
    meta::{credentials::CredentialsProviderChain, region::RegionProviderChain},
    profile::{ProfileFileCredentialsProvider, ProfileFileRegionProvider},
    BehaviorVersion, Region,
};
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ContentBlockDelta, ConversationRole, ConverseStreamOutput,
    InferenceConfiguration, Message,
};

pub struct LargeLanguageModel {
    #[expect(dead_code)]
    aws_config: aws_config::SdkConfig,
    bedrock_runtime_client: aws_sdk_bedrockruntime::Client,
    #[expect(dead_code)]
    bedrock_client: aws_sdk_bedrock::Client,
    inference_parameters: InferenceConfiguration,
    model_id: String,
}

const MODELS: &[(&str, &str)] = &[
    ("ClaudeV2", "anthropic.claude-v2"),
    ("ClaudeV21", "anthropic.claude-v2:1"),
    ("ClaudeV3Haiku", "anthropic.claude-3-haiku-20240307-v1:0"),
    ("ClaudeV3Sonnet", "anthropic.claude-3-sonnet-20240229-v1:0"),
    (
        "ClaudeV35Sonnet",
        "anthropic.claude-3-5-sonnet-20240620-v1:0",
    ),
    ("Llama270b", "meta.llama2-70b-chat-v1"),
    ("CohereCommand", "cohere.command-text-v14"),
    ("Jurrasic2Ultra", "ai21.j2-ultra-v1"),
    ("TitanTextExpressV1", "amazon.titan-text-express-v1"),
    ("Mixtral8x7bInstruct", "mistral.mixtral-8x7b-instruct-v0:1"),
    ("Mistral7bInstruct", "mistral.mistral-7b-instruct-v0:2"),
    ("MistralLarge", "mistral.mistral-large-2402-v1:0"),
    ("MistralLarge2", "mistral.mistral-large-2407-v1:0"),
    ("JambdaV15Large", "ai21.jamba-1-5-large-v1:0"),
];

impl LargeLanguageModel {
    pub async fn new(model_id: Option<&str>, region: Option<&str>) -> anyhow::Result<Self> {
        let model_id = Self::lookup_model_id(model_id)?;
        let region = region.unwrap_or("us-east-1");

        let aws_config = Self::aws_config(region, "default").await;
        let bedrock_runtime_client = aws_sdk_bedrockruntime::Client::new(&aws_config);
        let bedrock_client = aws_sdk_bedrock::Client::new(&aws_config);
        let inference_parameters = InferenceConfiguration::builder().build();
        Ok(Self {
            aws_config,
            bedrock_runtime_client,
            bedrock_client,
            inference_parameters,
            model_id,
        })
    }

    fn lookup_model_id(model_id: Option<&str>) -> anyhow::Result<String> {
        let Some(s) = model_id else {
            return Self::lookup_model_id(Some("JambdaV15Large"));
        };

        if s.contains(".") {
            return Ok(s.to_string());
        }

        for &(key, value) in MODELS {
            if key == s {
                return Ok(value.to_string());
            }
        }

        anyhow::bail!(
            "unknown model-id; try one of the following: [{}]",
            MODELS
                .iter()
                .map(|&(k, _)| k)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    pub async fn query(&self, prompt: &str, query: &str) -> anyhow::Result<String> {
        use std::fmt::Write;

        let mut output = self
            .bedrock_runtime_client
            .converse_stream()
            .model_id(&self.model_id)
            .messages(
                Message::builder()
                    .role(ConversationRole::Assistant)
                    .content(ContentBlock::Text(prompt.to_string()))
                    .role(ConversationRole::User)
                    .content(ContentBlock::Text(query.to_string()))
                    .build()
                    .with_context(|| "failed to build message")?,
            )
            .inference_config(self.inference_parameters.clone())
            .send()
            .await?;

        let mut result = String::new();
        loop {
            let token = output.stream.recv().await?;
            match token {
                Some(ConverseStreamOutput::ContentBlockDelta(event)) => match event.delta() {
                    Some(ContentBlockDelta::Text(text)) => write!(result, "{text}")?,
                    Some(delta) => panic!("unexpected response from bedrock: {delta:?}"),
                    None => (),
                },

                Some(_) => { /* ignore other messages */ }

                None => break,
            }
        }

        Ok(result)
    }

    async fn aws_config(fallback_region: &str, profile_name: &str) -> aws_config::SdkConfig {
        let region_provider = RegionProviderChain::first_try(
            ProfileFileRegionProvider::builder()
                .profile_name(profile_name)
                .build(),
        )
        .or_else(aws_config::environment::EnvironmentVariableRegionProvider::new())
        .or_else(aws_config::imds::region::ImdsRegionProvider::builder().build())
        .or_else(Region::new(fallback_region.to_string()));

        let credentials_provider = CredentialsProviderChain::first_try(
            "Environment",
            EnvironmentVariableCredentialsProvider::new(),
        )
        .or_else(
            "Profile",
            ProfileFileCredentialsProvider::builder()
                .profile_name(profile_name)
                .build(),
        )
        .or_else("IMDS", ImdsCredentialsProvider::builder().build());

        aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(credentials_provider)
            .region(region_provider)
            .load()
            .await
    }
}
