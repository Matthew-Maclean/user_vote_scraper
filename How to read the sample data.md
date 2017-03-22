The sample data in `sample_data.json` is the result of a GET to `reddit.com/user/[redacted]/comments/.json?limit=1`. All irrelevant information has been redacted.
The only bits this program cares about is the `likes` property, which can be `true` (upvoted), `false` (downvoted), or `null` (not voted on).
The path to this `data.children[index].data.likes`.

The data in `sample_post.json` is the result of a GET to `reddit.com/[redacted]/submitted/.json?limit=1`. It's mostly the same as the sample comments.
The `likes` property is in the same place and behaves the same.