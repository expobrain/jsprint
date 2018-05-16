#!/usr/bin/env python3

from operator import attrgetter
from pathlib import Path
import traceback
import functools
import getpass
import cmd
import shlex
import json

from colorama import Fore, Style, init, deinit
from jira import JIRA


UNASSIGNED = "<unassigned>"
ISSUE_STATUS_COLOR = {
    "Backlog": Fore.BLUE,
    "Open": Fore.RED,
    "In Progress": Fore.RED,
    "Reopened": Fore.RED,
    "On Review": Fore.YELLOW,
    "Resolved": Fore.GREEN,
    "Closed": Fore.GREEN,
    "On Production": Fore.GREEN,
    "In Build - Ok": Fore.GREEN,
}


settings = {}

with open(Path(__file__).parents[0] / "settings.json") as f:
    settings = json.load(f)


class JSprintException(Exception):
    pass


def do_exception(fn):

    def _wrapper(*args, **kwds):
        try:
            return fn(*args, **kwds)
        except JSprintException as e:
            traceback.print_tb(e.__traceback__)
            print(Fore.RED + Style.BRIGHT + f"ERROR: {e}" + Style.RESET_ALL)
            return

    return _wrapper


def get_issue_key_from_number(v) -> str:
    try:
        issue_number = int(v)
    except ValueError:
        raise JSprintException("Issue number is not a number")

    jira_project = settings.get("jira_project")
    issue_key = f"{jira_project}-{issue_number}"

    return issue_key


def get_print_id_from_number(v) -> str:
    try:
        return int(v)
    except ValueError:
        raise JSprintException("Sprint ID is not a number")


class JSprint(cmd.Cmd):

    jira = None

    def __init__(self):
        super().__init__()

        username = settings.get("jira_username")
        password = settings.get("jira_password") or getpass.getpass()

        options = {"server": settings.get("jira_url"), "agile_rest_path": "agile"}

        self.jira = JIRA(options, auth=(username, password))

    def get_active_sprint(self):
        sprints = self.jira.sprints(settings.get("jira_board_id"), state="active")

        if len(sprints):
            # Use fisrt active sprint as default
            sprint = sorted(sprints, key=attrgetter("name"))[0]
        else:
            sprint = None

        return sprint

    # ----------------
    # Fetch sprints
    # ----------------
    def do_sps(self, line):
        return self.do_sprints(line)

    @do_exception
    def do_sprints(self, line):
        actives = self.jira.sprints(settings.get("jira_board_id"), state="active")
        futures = self.jira.sprints(settings.get("jira_board_id"), state="future")

        sprints = futures + actives
        sprints = sorted(sprints, key=attrgetter("name"))

        for sprint in sprints:
            name = sprint.name
            id_ = Style.BRIGHT + str(sprint.id) + Style.RESET_ALL
            state = (
                Style.BRIGHT
                + (Fore.YELLOW if sprint.state == "future" else Fore.GREEN)
                + ("*" if sprint.state == "active" else " ")
                + sprint.state
                + Style.RESET_ALL
            )

            print(f"{state} {name} ({id_})")

    def do_sp(self, line):
        return self.do_sprint(line)

    @do_exception
    def do_sprint(self, line):

        def group_by_assignee(acc, issue):
            assignee = issue.fields.assignee.displayName if issue.fields.assignee else UNASSIGNED

            acc.setdefault(assignee, []).append(issue)

            return acc

        # Parse arguments
        args = shlex.split(line)

        # Show sprint
        if len(args) == 0:
            sprint = self.get_active_sprint()

            if sprint is None:
                print("No active sprint")
                return
            else:
                sprint_id = sprint.id

            print(f"Displaying sprint {sprint.name}")
        else:
            sprint_id = get_print_id_from_number(args[0])

        # Show issue by assignee
        jql = f"""
            project = '{settings.get('jira_project')}' AND
            sprint = {sprint_id} AND
            (assignee IS NULL or assignee IN {tuple(settings.get('team_members'))}) AND
            (labels IS NULL or labels IN {tuple(settings.get('team_labels'))})
        """

        issues = self.jira.search_issues(jql)

        parmalink_padding = max(len(i.permalink()) for i in issues)
        status_padding = max(len(i.fields.status.name) for i in issues)

        issues_by_user = functools.reduce(group_by_assignee, issues, {})
        assignees = sorted(issues_by_user.keys())

        for i, assignee in enumerate(assignees):
            user_issues = issues_by_user[assignee]
            user_issues = sorted(user_issues, key=attrgetter("fields.status.name"))

            print(Style.BRIGHT + f"{assignee}:" + Style.RESET_ALL)

            for issue in user_issues:
                url = issue.permalink().ljust(parmalink_padding)
                status = issue.fields.status.name
                status_color = ISSUE_STATUS_COLOR.get(status, "")
                status = Style.BRIGHT + status_color + status.ljust(
                    status_padding
                ) + Style.RESET_ALL
                summary = Style.BRIGHT + issue.fields.summary + Style.RESET_ALL

                print(f"{status} - {url} {summary}")

            if i != (len(assignees) - 1):
                print()

    # --------------------
    # Assign user to issue
    # --------------------
    def do_a(self, line):
        return self.do_assign(line)

    @do_exception
    def do_assign(self, line):
        # Parse args
        args = shlex.split(line)

        if len(args) < 2:
            print("Need two arguments: issue number and assignee name")
            return

        # Assign issue
        issue_key = get_issue_key_from_number(args[0])
        assignee = args[1]

        self.jira.assign_issue(issue_key, assignee)

    # ----------------------
    # Unassign user to issue
    # ----------------------
    def do_u(self, line):
        return self.do_unassign(line)

    @do_exception
    def do_unassign(self, line):
        # Parse args
        args = shlex.split(line)

        if len(args) < 1:
            print("Needs at least an issue number")
            return

        # Unassign issue
        issue_key = get_issue_key_from_number(args[0])

        self.jira.assign_issue(issue_key, None)

    # -------------------
    # Add issue to sprint
    # -------------------
    @do_exception
    def do_add(self, line):
        # Parse args
        args = shlex.split(line)

        if len(args) == 0:
            print("Needs at least an issue number")
            return

        # Get active sprint
        issue_key = get_issue_key_from_number(args[0])
        sprint = self.get_active_sprint()

        # Add issue to sprint
        self.jira.add_issues_to_sprint(sprint.id, [issue_key])

    # ----------------
    # Leave the REPL
    # ----------------
    def do_quit(self, line):
        return True

    def do_q(self, args):
        return True

    def do_EOF(self, line):
        return True


if __name__ == "__main__":
    init()
    JSprint().cmdloop()
    deinit()
