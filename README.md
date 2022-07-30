# PrettyMR
Takes Gitlab Merge Request URLs as input along with the corresponding issue names and generates a message for messaging platforms

## Usage

```
./prettymr [IssueName1]: [URL] [URL] ... [URL] [IssueName2]: [URL] [URL] ... [URL] ...
```

### Example

```
./prettymr Issue-134: https://gitlab.com/username.com/bla/-/merge_requests/13 https://gitlab.com/username.com/foo/-/merge_requests/132 Issue-21: https://gitlab.com/username.com/foo/-/merge_requests/153
```

This will produce:
```
MR Issue-134 (bla: [!13](https://gitlab.com/username.com/bla/-/merge_requests/13), foo: [!132](https://gitlab.com/username.com/foo/-/merge_requests/132)) Issue-21 (foo: [!153](https://gitlab.com/username.com/foo/-/merge_requests/153))
```

Example Output in Slack

![Slack Demo](resources/slack-demo.png)

## Configuration

```
./prettymr --setprefix [IssuePrefix]
```

### Example

```
./prettymr --setprefix Issue-
```

This prefixes 'Issue-' to all issue names. For example this
```
./prettymr 134: https://gitlab.com/username.com/bla/-/merge_requests/13 https://gitlab.com/username.com/foo/-/merge_requests/132 21: https://gitlab.com/username.com/foo/-/merge_requests/153
```

will produce
```
MR Issue-134 (bla: [!13](https://gitlab.com/username.com/bla/-/merge_requests/13), foo: [!132](https://gitlab.com/username.com/foo/-/merge_requests/132)) Issue-21 (foo: [!153](https://gitlab.com/username.com/foo/-/merge_requests/153))
```

```
./prettymr --unsetprefix
```

This removes the prefix you set previously
