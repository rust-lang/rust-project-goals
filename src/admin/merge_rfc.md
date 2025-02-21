# Merging the RFC

Once the RFC is accepted

* Merge the RFC itself
* Run `cargo rpg teams` to prepare adjustment to the teams repository
    * In particular to populate the project-goal-owners team 
    * This will be needed to allow people to author updates on their goals
* Run `cargo rpg issues` to create and update tracking issues
    * Continuing goals will be moved to the new milestone
    * You can run this tool over and over to keep things synchronized
* Close out of date issues
    * For all issues in previous milestone, close them