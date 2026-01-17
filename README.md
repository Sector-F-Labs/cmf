# CMF - Conversational Markdown Format

A markdown-based interchange format for LLM conversations. User messages are blockquotes (`>`), assistant messages are plain markdown.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Detect CMF content and count turns
cmf detect conversation.cmf

# Check conformance (silent on success)
cmf check conversation.cmf

# Convert to OpenAI Chat Completions format
cmf to-openai-chat conversation.cmf

# Convert to OpenAI Responses API format
cmf to-openai-responses conversation.cmf
```

## Format

```markdown
> @alice: Can you explain the steps?
Yes. First you need to set up your environment.

Second, assistant messages can be multi-paragraph.

 > Indent blockquotes to avoid parsing as user lines.

> Thanks!
Glad it helped.
```

**Rules:**
- User lines start with `>` in column 1
- Multi-user chats use `> @username:` prefix
- Assistant content is everything between user blocks
- Indent blockquotes (` > text`) to escape them in assistant content

## Library

```rust
use cmf::Document;

let doc = Document::parse(input);
for turn in &doc.turns {
    println!("User: {}", turn.user.content);
    println!("Assistant: {}", turn.assistant);
}

// Convert to OpenAI formats
let chat_messages = doc.to_openai_chat();
let responses_messages = doc.to_openai_responses();
```

## License

BSD-3-Clause
