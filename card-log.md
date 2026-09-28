#card log (Partner B)

##Card 1
Commit message did not follow conventional standards ->
used git commit --amend -> replaced last commit instead
of creating a new one.

##Card 2
added all files using git add .-> unstaged notes.txt
by using git restore-> removed using git rm notes.txt -f

##Card 3
commited private information-> removed commit using 
git reset --soft(undid commit but left staged)-> 
used git reset (undid commit and unstaged)->
used git reset --hard (undid commit and deleted files)
 
##Card 4
commitedand pushed faulty code-> un(commited/pushed)
using git revert HEAD

##Card 5
created scratch.txtecho "one" and "two"->moved head to 
before the echos were commited, removing them-> reverted
this change by doing a hard reset at the logged point for 
when the echos were commited. 
