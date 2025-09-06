#!/bin/bash

echo "🔧 Fixing all remaining syntax errors..."

# Fix the pattern: let class = merge_classes([\n        "string",\n        }\n    };
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let class = merge_classes(\[\n        "\([^"]*\)",\n        }\n    };/let class = merge_classes([\n        "\1",\n    ]);/g' {} \;

# Fix the pattern: let class = merge_classes([\n        "string",\n        </div>\n    }
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let class = merge_classes(\[\n        "\([^"]*\)",\n        <\/div>\n    }/let class = merge_classes([\n        "\1",\n    ]);/g' {} \;

# Fix the pattern: let class = merge_classes([\n        "string",\n        }\n    };
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let class = merge_classes(\[\n        "\([^"]*\)",\n        }\n    };/let class = merge_classes([\n        "\1",\n    ]);/g' {} \;

echo "✅ All syntax errors fixed!"
