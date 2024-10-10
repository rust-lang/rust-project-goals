(function () {
    document.addEventListener("DOMContentLoaded", function () {
        updateProgressBars();
    });

    // Searches the document for `<progress>` elements.
    //
    // The `id` is expected to have the format `2024h2:rust-lang:rust-project-goals:123`
    async function updateProgressBars() {
        let issueData = new IssueData();
        document.querySelectorAll('div.tracking-issue-progress').forEach(async progressDiv => {
            const id = progressDiv.id;
            if (!id) {
                console.error("progress element is missing an id");
                return;
            }

            try {
                const issue = await issueData.loadData(id);
                if (issue) {
                    progressDiv.append(issue.progress());
                }
            } catch (error) {
                console.error(`Error loading data for ${id}:`, error.message);
            }
        });
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
                try {
                    const response = await fetch(getApiUrl(milestone));
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

        progress() {
            let state = this.#json.state;

            function progressElement(completed, total) {
                // If the issue is closed, then either the work is COMPLETE
                // or will never complete.
                if (state === "CLOSED") {
                    if (completed === total) {
                        return element("center", element("img", { src: "https://img.shields.io/badge/Completed!%20%3A%29-green", alt: "Completed" }));
                    } else {
                        return element("center", element("img", { src: "https://img.shields.io/badge/Incomplete%20%3A%28-yellow", alt: "Incomplete" }));
                    }
                } else {
                    return element("progress", { value: completed, max: total });
                }
            }

            let o = this.#json.progress.Tracked;
            if (o) {
                return progressElement(o.completed, o.total);
            }

            o = this.#json.progress.Binary;
            if (o) {
                if (state === "OPEN") {
                    return progressElement(0, 1);
                } else {
                    return progressElement(1, 1);
                }
            }

            o = this.#json.progress.Error;
            let message = o?.message || "Error loading status";
            return element("span", { title: message }, "⚠️");
        }
    }

    function element(tag, attrs, ...children) {
        const element = document.createElement(tag);

        if (typeof attrs === 'string' || (typeof attrs === 'object' && 'nodeType' in attrs)) {
            children.unshift(attrs);
        } else if (attrs && typeof attrs === 'object') {
            for (const [key, value] of Object.entries(attrs)) {
                element.setAttribute(key, value);
            }
        }

        for (const child of children) {
            if (typeof child === 'string') {
                element.append(document.createTextNode(child));
            } else {
                element.append(child);
            }
        }
        return element;
    }

    function getApiUrl(milestone) {
        // When you are doing mdbook serve, you don't want /rust-project/goals.
        // But on github pages, you do.
        // There is probably a more elegant way to do this.
        const currentPath = document.location.pathname;
        const basePath = currentPath.startsWith('/rust-project-goals/')
            ? '/rust-project-goals/'
            : '/';
        return `${basePath}api/${milestone}.json`;
    }
})();