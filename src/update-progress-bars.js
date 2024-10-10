document.addEventListener("DOMContentLoaded", function () {
    updateProgressBars();
});

// Searches the document for `<progress>` elements.
//
// The `id` is expected to have the format `2024h2:rust-lang:rust-project-goals:123`
async function updateProgressBars() {
    let issueData = new IssueData();
    for (let progressBar of document.body.getElementsByTagName("progress")) {
        const id = progressBar.id;
        if (!id) {
            console.error("progress element is missing an id");
            continue;
        }

        try {
            const issue = await issueData.loadData(id);
            if (issue) {
                progressBar.value = issue.completedItems();
                progressBar.max = issue.totalItems();
            }
        } catch (error) {
            console.error(`Error loading data for ${id}:`, error.message);
        }
    }
}

class IssueData {
    #dataMap = {};

    constructor() {
    }

    async loadData(id) {
        // Split the id into four parts
        const [milestone, org, repo, issue] = id.split(':');

        if (!milestone || !org || !repo || !issue) {
            throw new Error(`id ${id} does not have the expected format: dirName:org:repo:issue`);
        }

        if (!(milestone in this.#dataMap)) {
            // Construct the URL using the dirName
            const url = `/api/${milestone}.json`;

            try {
                const response = await fetch(url);
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }

                const data = await response.json();
                this.#dataMap[milestone] = data;
            } catch (error) {
                console.error(`error loading data for ${id} from ${url}:`, error.message);
                throw error;
            }
        }

        return this.#findData(milestone, org, repo, issue);
    }

    #findData(milestone, org, repo, issueString) {
        const milestoneJson = this.#dataMap[milestone];
        const repository = `${org}/${repo}`;
        let issueNumber = parseInt(issueString);

        if (milestoneJson.repository !== repository) {
            throw new Error(`expected repository ${repository} but found ${milestoneJson.repository}`);
        }

        for (let issueJson of milestoneJson.issues) {
            if (issueJson.number === issueNumber) {
                return new Issue(issueJson);
            }
        }

        return undefined;
    }
}

class Issue {
    #json;

    constructor(json) {
        this.#json = json;
    }

    completedItems() {
        return this.#json.checked_checkboxes;
    }

    totalItems() {
        return this.#json.total_checkboxes;
    }
}