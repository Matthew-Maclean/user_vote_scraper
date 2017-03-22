The sample data in `sample_data.json` is the result of a GET to `reddit.com/user/[redacted]/comments/.json?limit=3`. All irrelevant information has been redacted.
The only bits this program cares about is the `likes` property, which can be `true` (upvoted), `false` (downvoted), or `null` (not voted on).
The path to this `data.children[index].data.likes`.