# neo-space
Infinite Canvas in Rust Webassembly. 
Draw, diagram, generate, animate, share. Local or in browser. 
NO CORPORATE MIDDLEMEN.

# helpful git commands for contributing
fork the original repo, download it, and work off a new branch
```bash
git checkout -b name-that-implies-your-change
```
verify that you are tracking the remote
```bash
git remote -v
```
you might be tracking your own remote, add neo-space/neo-space as a remote
```bash
git remote add upstream https://github.com/neo-space/neo-space.git
```
check your `origin` and `upstream`, you should see something like this.
origin is your fork and neo-space is the original
```bash
origin  git@github.com:vanitysemptiness/neo-space.git (fetch)
origin  git@github.com:vanitysemptiness/neo-space.git (push)
upstream        https://github.com/neo-space/neo-space.git (fetch)
upstream        https://github.com/neo-space/neo-space.git (push)
```

fetch latest changes from upstream repository
```bash
git fetch upstream
```
change to your main branch, ditch your changes in main (which should be none as you use branches), and reset to upstream/main
```bash
git checkout main
git reset --hard upstream/main
```
push your updated main to your forks main 
```bash
git push origin main --force
```
branching, making changes, and pushing them to your fork (`origin`)
```bash
git checkout -b my-new-feature
# Make your changes, commit them
git push origin my-new-feature
```
Once thats there, raise the PR against neo-space/neo-space on github
