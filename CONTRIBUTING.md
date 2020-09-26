# Contribution Guidelines

Hi!

Please read the guidelines below before submitting contributions to sflynlang/compiler.

More help can be found by joining our [Official Discord Server](https://discord.gg/XdeRFHt).

## Steps if you wish to contribute to Sflynlang:
1. First review [Issues](https://github.com/sflynlang/compiler/issues) to see if what you have in mind is already under discussion. (**NOTE** If you are using GitHub CLI, you can run `gh issue view -R sflynlang/compiler`. [See more](https://cli.github.com/manual/gh_issue_view))

2. If your issue is not already addressed in [Issues](https://github.com/sflynlang/compiler/issues), you can open a new Issue to be reviewed. Please provide as much description as you can for the bug, feature request, problem etc in the Issue message.

3. Mention in the Issue that you want to work on it and ask to be assigned.

4. Wait for the approval from the Maintainers of this project before creating a new branch to start work on it.

5. Create a pull request after making the changes and mention the Issue number that your pull request is related to.

6. Make the required changes if the reviewer asks for them.

7. That's it! Your pull request will be merged once everything is okay. (:

**Watch this video if you are new in GitHub** [YouTube Video](https://youtu.be/HbSjyU2vf6Y)

## How to work in this project if you are not a member

### 1. Fork the repository
Fork this repository ([sflynlang/compiler](https://github.com/sflynlang/conpiler)) to your GitHub account. You can do this by clicking on the Fork button provided in the top right corner of the repository page.

![Fork Image](./assets/readme/fork_image.png)

> NOTE: If you are using GitHub CLI can run `gh repo fork sflynlang/compiler` to fork it. ([See more](https://cli.github.com/manual/gh_repo_fork))

Please also, star and watch this repository to receive all the updates directly to your e-mail.

**Watch this video to learn how to Fork a repository** [YouTube Video](https://youtu.be/HbSjyU2vf6Y?t=101)

### 2. Clone the forked repository to your system
Clone the repository that you just forked into your account. Be careful to clone the forked repository (your-username/compiler) and not the main repository (sflynlang/compiler) as making direct changes to the main repository will result in conflicts and harder change management in the future.

The forked repository will have your username in the top left corner and the clone link will also contain your username (https://github.com/your-username/compiler.git).

> NOTE: If you are using GitHub CLI can run `gh repo clone your-username/compiler` to clone your forked repository. ([See more](https://cli.github.com/manual/gh_repo_clone))

**Watch this video to learn how to clone a forked repository** [YouTube Video](https://youtu.be/HbSjyU2vf6Y?t=134)

### 3. Add the main repository as remote upstream
At this point, you should have the repository on your system and be ready to make changes. But what if someone else changes the same thing that you just did?

To avoid any merge conflicts, you need to pull all the changes from the main repository.

So, after cloning the forked repository (your-username/compiler) to your system, use the command `git remote add upstream https://github.com/sflynlang/compiler.git` to point git to the main repository. This only needs to be done once.

Now, after making any changes to your project on the system, follow the steps below to push your work to the repository:

`git add .`

`git commit -m "feature/fix/refactor/docs: Commit message"` (Remember to follow our [Code of Conduct](./CODE_OF_CONDUCT.md) and [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format)

`git pull upstream main` (This command checks for any conflicts with the main repository. Go through the conflicts and make changes, if required)

`git push origin main`

**Watch this video to learn how to set up a remote repository** [YouTube Video](https://youtu.be/-zvHQXnBO6c)

### 4. Read the official documentation
If you do not know how to install this compiler, please read this [installation guide](https://github.com/sflynlang/compiler/wiki#installation).

### 5. Create a pull request
After pushing your changes to your forked repository (your-username/compiler), create a new pull request from your account by simply clicking on the Pull Request button.

> NOTE: If some of your commits are behind _sflynlang/compiler:main_, then you need to first pull `git pull upstream main` from the system, push it to your forked repository and then create the Pull Request.

Give a detailed and useful explanation of what you did in the comments of your pull requests. We will review your code, accept and merge the pull request, or ask you to make any required changes.

> NOTE: If you are using GitHub CLI, please run `gh pr create --title "feature/fix: Title" --body "Pull request message"` or `gh pr create`. ([See more](https://cli.github.com/manual/gh_pr_create))

You can use our pull request template ([view here](./.github/PULL_REQUEST_TEMPLATE.md)).

**Watch this video to lewarn how to create a Pull Request** [YouTube Video](https://youtu.be/HbSjyU2vf6Y?t=297)

## Attribution
Many thanks to [Skill Board](https://github.com/devscollab/skill-board) project for the base of our Contribution Guidelines.

**🎉 Good luck and keep coding!**
