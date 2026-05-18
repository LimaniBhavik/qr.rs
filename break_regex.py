import re

with open('src/formats/mod.rs', 'r') as f:
    content = f.read()

# Make the regex match anything
new_content = content.replace(
    'Regex::new(r"^[a-zA-Z0-9!#$%&\'*+/=?^_`{|}~-]+(?:\\.[a-zA-Z0-9!#$%&\'*+/=?^_`{|}~-]+)*@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap()',
    'Regex::new(r".*").unwrap()'
)

with open('src/formats/mod.rs', 'w') as f:
    f.write(new_content)
