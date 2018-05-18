# JSprint

## Description

JSprint is a small command line utility written in Python to easly manage issues and sprints in
Jira.

It's written around my specific managing needs but feel free to copy and chnage it to better fit
you needs.

## Installation

JSprint needs Python 3.6+ to run.

Clone the repository, install the requirements and execute the CLI utility:

```
git clone https://github.com/expobrain/jsprint.git
cd jsprint
pip install -r requirements.txt
./jsprint.py
```

## Setup

Create a copy of `settings.sample.json` and save it as `settings.json` then fill in all the
settings relative for your setup:

* _jira_url_: URL of the Jira instance
* _jira_username_ and _jira_password_: credentials of the Jira's account
* _jira_project_: project's prefix to be used for issue's numbers
* _jira_board_id_: the Jira board's ID which contains the sprints
* _team_members_: optional list of the team members to be used to filter the sprint's results
* _team_labels_: optional list of labels to filter the sprint's results

## Commands

For the full list of available commands at the prompt type:

```
help
```
