# Git workflow

## General setup
. Fork the repo. In most git frontends, that fork will end up at
```
	git@github.com:<username>/libfrizz.git
```
. Clone the original fork and add a remote for the upstream to the repo ::
```
    $ git clone git@github.com:<username>/libfrizz.git
    $ cd libfrizz
    $ git remote add upstream git@github.com:kursatkobya/libfrizz.git
```
This will make the fork have `origin` as remote name, as it is the repository
the developer will typically push to.  On the other hand, we recommend to
always type the remote and branch name when pushing, which forces the developer
to be more explicit about their intentions.

If you did not manage to clone the repo via the steps above, it is most likely due to that the SSH
key not being set up. To fix this quickly
```
    $ ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
    # This will generate a new ssh key for you

    $ sudo apt install xclip
    # Downloads and installs xclip

    $ xclip -sel clip < ~/.ssh/id_rsa.pub
    # Copies the contents of the id_rsa.pub file to your clipboard
```
Then add your new SSH key to the account on the git web frontend.
A long guide (for GitHub) can be found here: https://help.github.com/articles/connecting-to-github-with-ssh/

##For each set of changes
. Check out the main branch and make sure it is up to date.
```
   $ git checkout main
   $ git pull upstream main
```
. Check out a new branch to work on ::
```
   $ git checkout -b some_branch_name
```
   This is optional, it is possible to work on main as well, but it has the downside of making
   main blocked while waiting for merge.

. Do some work, and commit it. 
. Make sure your branch is up-to-date with main
```
   $ git pull upstream main --rebase
```
   This is to make sure that any conflicts that exist before pushing are resolved locally before
   pushing. It might be the case that there are more people working on the same code that has a
   pull request which just got accepted, which would lead to a conflict in main. Then it is
   better to resolve the conflict in the local branch before creating a new pull request. To keep
   the git log clear of empty merge commits, we recommend rebasing.

. Push to the forked remote
```
   $ git push origin some_branch_name
```
. Create a pull request in github 

Make sure the rest of the team is notified of the pull request. There are different ways of handling
changes to the pull request. Either any new changes can be added to an existing commit and
force-pushed to the forked repo, or they can be added on top of the existing commit. The pull
request will be updated either way.

Whenever the team is satisfied with the changes, pull the changes in by accepting the pull request.
