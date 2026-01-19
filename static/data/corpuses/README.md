# Language Corpuses

This directory contains JSON files with language-specific keywords, functions, and identifiers for the autocomplete system.

## Structure

Each corpus file follows this format:

```json
{
  "language": "language-name",
  "entries": [
    {
      "text": "keyword_or_function",
      "type": "keyword|function|type|constant|variable",
      "description": "Optional description",
      "category": "Optional category"
    }
  ]
}
```

## Entry Types

- **keyword**: Language keywords (if, for, class, def, etc.)
- **function**: Functions and methods (print, map, filter, etc.)
- **type**: Types and classes (String, Array, int, etc.)
- **constant**: Constants (true, false, null, Math.PI, etc.)
- **variable**: Common variable names (i, j, index, result, etc.)

## Supported Languages

### High Priority (Preloaded)
- javascript.json
- typescript.json
- python.json
- c.json
- cpp.json

### Medium Priority (Lazy Loaded)
- html.json
- css.json
- sql.json
- rust.json
- go.json
- java.json

### Low Priority (Lazy Loaded)
- assembly.json
- bash.json
- makefile.json
- yaml.json
- json.json
- toml.json
- markdown.json
- And others...

## Adding New Corpuses

1. Create a new JSON file named `{language}.json`
2. Follow the structure above
3. Include relevant keywords, functions, and types for the language
4. Keep entries focused on commonly used items (aim for 100-600 entries)
5. Add descriptions for functions and types to help users

## Performance Considerations

- Each corpus should contain 100-600 entries for optimal performance
- Total memory budget for all corpuses: 50MB
- Corpuses are lazy-loaded on first use
- Common languages are preloaded on IDE startup
