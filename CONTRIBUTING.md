# CONTRIBUTING

## How to contribute

You should use the [Fork & Pull](https://guides.github.com/activities/forking/) method.
Steps: fork a repo -> make changes and commits -> send a pull request


## Style

See the [Rust Official Style Guide](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md).

## Commit messages

Follow [Angular Git Commit Guidelines](https://github.com/angular/angular.js/blob/master/CONTRIBUTING.md#-git-commit-guidelines).

Short & easy way:

1. Install `commitizen` with 
```
$ npm install -g commitizen
```

2. Configure commitizen with sane defaults
``` 
$ npm install -g cz-conventional-changelog
$ echo '{ "path": "cz-conventional-changelog" }' > ~/.czrc
```

3. **Always use** `git cz` when commiting instead of `git commit`

4. Done!

(You can always manually type all those pesky parentheses and colons if you wish instead
 of using an automated tool. As long as commit style is consistent I won't complain. Maybe.)
