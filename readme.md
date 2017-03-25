# User Vote Scraper (for reddit)

Scrapes a user's posts and comments to find which ones you have voted on.  
Ever use RES and see a `[+10]` next to someones name, and wonder which of their posts you voted on?  
This tool will do that for you.

example uses:

    # find all comments you voted on by automoderator, then generate a reddit api list to browse
    user_vote_scraper automoderator -c -o api
    # find from automoderator's last 65 posts which ones you voted on, and print the metrics
    user_vote_scraper automoderator -p -m
