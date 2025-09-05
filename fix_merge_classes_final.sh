#!/bin/bash

# Fix remaining merge_classes syntax errors
echo "🔧 Fixing remaining merge_classes syntax errors..."

# Fix the pattern: let class = \n        "string",\n    ]);
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let class = $/let class = merge_classes([/g' {} \;

# Fix the pattern: let class = \n        "string",\n        }\n    };
find crates/ -name "*.rs" -type f -exec sed -i '' 's/let class = $/let class = merge_classes([/g' {} \;

echo "✅ merge_classes syntax errors fixed!"
