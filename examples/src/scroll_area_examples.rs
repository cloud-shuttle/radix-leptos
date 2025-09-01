use leptos::prelude::*;
use radix_leptos_primitives::components::*;

#[component]
pub fn ScrollAreaExamples() -> impl IntoView {
    // Generate sample content for different scroll areas
    let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. ".repeat(20);

    let wide_content = (1..=50).map(|i| format!("Column {}", i)).collect::<Vec<_>>().join(" | ");

    // Create multiple instances of table data to avoid ownership issues
    let table_data_1 = (1..=100).map(|i| {
        format!("Row {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", i)
    }).collect::<Vec<_>>();
    
    let table_data_2 = (1..=100).map(|i| {
        format!("Row {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", i)
    }).collect::<Vec<_>>();
    
    let table_data_3 = (1..=100).map(|i| {
        format!("Row {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", i)
    }).collect::<Vec<_>>();
    
    let table_data_4 = (1..=100).map(|i| {
        format!("Row {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", i)
    }).collect::<Vec<_>>();
    
    let table_data_5 = (1..=100).map(|i| {
        format!("Row {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", i)
    }).collect::<Vec<_>>();
    
    let table_data_6 = (1..=100).map(|i| {
        format!("Row {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.", i)
    }).collect::<Vec<_>>();

    let code_content = r#"
function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

function factorial(n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

function bubbleSort(arr) {
    const len = arr.length;
    for (let i = 0; i < len; i++) {
        for (let j = 0; j < len - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                [arr[j], arr[j + 1]] = [arr[j + 1], arr[j]];
            }
        }
    }
    return arr;
}

function quickSort(arr) {
    if (arr.length <= 1) return arr;
    
    const pivot = arr[Math.floor(arr.length / 2)];
    const left = arr.filter(x => x < pivot);
    const middle = arr.filter(x => x === pivot);
    const right = arr.filter(x => x > pivot);
    
    return [...quickSort(left), ...middle, ...quickSort(right)];
}

function mergeSort(arr) {
    if (arr.length <= 1) return arr;
    
    const mid = Math.floor(arr.length / 2);
    const left = mergeSort(arr.slice(0, mid));
    const right = mergeSort(arr.slice(mid));
    
    return merge(left, right);
}

function merge(left, right) {
    const result = [];
    let i = 0, j = 0;
    
    while (i < left.length && j < right.length) {
        if (left[i] <= right[j]) {
            result.push(left[i]);
            i++;
        } else {
            result.push(right[j]);
            j++;
        }
    }
    
    return result.concat(left.slice(i), right.slice(j));
}
"#.repeat(5);

    view! {
        <div class="min-h-screen bg-gray-50 p-8">
            <div class="max-w-6xl mx-auto space-y-8">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-gray-900 mb-4">"ScrollArea Component Examples"</h1>
                    <p class="text-lg text-gray-600">"Custom scrollable areas with enhanced styling and functionality"</p>
                </div>

                // Basic ScrollArea Examples
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Basic ScrollArea"</h2>
                    <p class="text-gray-600 mb-4">"Simple scrollable areas with different orientations"</p>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        // Vertical ScrollArea
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Vertical Scroll"</h3>
                            <ScrollArea 
                                orientation=ScrollAreaOrientation::Vertical 
                                class="h-64 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="space-y-2">
                                        {table_data_1.iter().take(20).map(|row| {
                                            view! {
                                                <div class="p-2 bg-gray-50 rounded text-sm">
                                                    {row.clone()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>

                        // Horizontal ScrollArea
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Horizontal Scroll"</h3>
                            <ScrollArea 
                                orientation=ScrollAreaOrientation::Horizontal 
                                class="h-32 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="whitespace-nowrap text-sm">
                                        {wide_content}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>
                    </div>
                </div>

                // Size Variants
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"ScrollArea Sizes"</h2>
                    <p class="text-gray-600 mb-4">"Different scrollbar sizes: Small, Medium, and Large"</p>
                    
                    <div class="space-y-4">
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Small Scrollbar"</h3>
                            <ScrollArea 
                                size=ScrollAreaSize::Small 
                                class="h-48 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="space-y-2">
                                        {table_data_2.iter().take(15).map(|row| {
                                            view! {
                                                <div class="p-2 bg-blue-50 rounded text-sm">
                                                    {row.clone()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Medium Scrollbar (Default)"</h3>
                            <ScrollArea 
                                size=ScrollAreaSize::Medium 
                                class="h-48 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="space-y-2">
                                        {table_data_3.iter().take(15).map(|row| {
                                            view! {
                                                <div class="p-2 bg-green-50 rounded text-sm">
                                                    {row.clone()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Large Scrollbar"</h3>
                            <ScrollArea 
                                size=ScrollAreaSize::Large 
                                class="h-48 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="space-y-2">
                                        {table_data_4.iter().take(15).map(|row| {
                                            view! {
                                                <div class="p-2 bg-purple-50 rounded text-sm">
                                                    {row.clone()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>
                    </div>
                </div>

                // Code Editor Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Code Editor ScrollArea"</h2>
                    <p class="text-gray-600 mb-4">"ScrollArea used in a code editor context"</p>
                    
                    <ScrollArea 
                        orientation=ScrollAreaOrientation::Both 
                        class="h-96 w-full border rounded-md bg-gray-900".to_string()
                    >
                        <ScrollAreaViewport>
                            <pre class="p-4 text-green-400 font-mono text-sm">
                                <code>{code_content}</code>
                            </pre>
                        </ScrollAreaViewport>
                    </ScrollArea>
                </div>

                // Long Text Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Long Text ScrollArea"</h2>
                    <p class="text-gray-600 mb-4">"ScrollArea for long text content"</p>
                    
                    <ScrollArea 
                        orientation=ScrollAreaOrientation::Vertical 
                        class="h-64 w-full border rounded-md p-4".to_string()
                    >
                        <ScrollAreaViewport>
                            <div class="prose prose-sm max-w-none">
                                <p class="text-gray-700 leading-relaxed">{long_text}</p>
                            </div>
                        </ScrollAreaViewport>
                    </ScrollArea>
                </div>

                // Data Table Example
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Data Table ScrollArea"</h2>
                    <p class="text-gray-600 mb-4">"ScrollArea for large data tables"</p>
                    
                    <ScrollArea 
                        orientation=ScrollAreaOrientation::Both 
                        class="h-80 w-full border rounded-md".to_string()
                    >
                        <ScrollAreaViewport>
                            <table class="w-full border-collapse">
                                <thead class="bg-gray-50 sticky top-0">
                                    <tr>
                                        <th class="border px-4 py-2 text-left font-semibold">"ID"</th>
                                        <th class="border px-4 py-2 text-left font-semibold">"Name"</th>
                                        <th class="border px-4 py-2 text-left font-semibold">"Email"</th>
                                        <th class="border px-4 py-2 text-left font-semibold">"Department"</th>
                                        <th class="border px-4 py-2 text-left font-semibold">"Status"</th>
                                        <th class="border px-4 py-2 text-left font-semibold">"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {(1..=50).map(|i| {
                                        view! {
                                            <tr class="hover:bg-gray-50">
                                                <td class="border px-4 py-2">{i}</td>
                                                <td class="border px-4 py-2">"User {i}"</td>
                                                <td class="border px-4 py-2">"user{i}@example.com"</td>
                                                <td class="border px-4 py-2">"Department {(i % 5) + 1}"</td>
                                                <td class="border px-4 py-2">
                                                    <span class="px-2 py-1 rounded text-xs {if i % 3 == 0 { \"bg-green-100 text-green-800\" } else if i % 3 == 1 { \"bg-yellow-100 text-yellow-800\" } else { \"bg-red-100 text-red-800\" }}">
                                                        {if i % 3 == 0 { "Active" } else if i % 3 == 1 { "Pending" } else { "Inactive" }}
                                                    </span>
                                                </td>
                                                <td class="border px-4 py-2">
                                                    <button class="text-blue-600 hover:text-blue-800 text-sm">"Edit"</button>
                                                </td>
                                            </tr>
                                        }
                                    }).collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </ScrollAreaViewport>
                    </ScrollArea>
                </div>

                // Custom Styled ScrollArea
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Custom Styled ScrollArea"</h2>
                    <p class="text-gray-600 mb-4">"ScrollArea with custom styling and colors"</p>
                    
                    <ScrollArea 
                        class="h-64 w-full border-2 border-blue-200 rounded-lg p-4 bg-gradient-to-br from-blue-50 to-indigo-50".to_string()
                    >
                        <ScrollAreaViewport>
                            <div class="space-y-3">
                                {(1..=30).map(|_i| {
                                    view! {
                                        <div class="p-3 bg-white rounded-lg shadow-sm border border-blue-100">
                                            <h4 class="font-medium text-blue-900">"Item {i}"</h4>
                                            <p class="text-blue-700 text-sm">"This is a custom styled item with blue theme"</p>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </ScrollAreaViewport>
                    </ScrollArea>
                </div>

                // Responsive ScrollArea
                <div class="bg-white rounded-lg shadow-md p-6">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-4">"Responsive ScrollArea"</h2>
                    <p class="text-gray-600 mb-4">"ScrollArea that adapts to different screen sizes"</p>
                    
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Mobile/Tablet View"</h3>
                            <ScrollArea 
                                class="h-48 lg:h-64 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="space-y-2">
                                        {table_data_5.iter().take(12).map(|row| {
                                            view! {
                                                <div class="p-2 bg-gray-50 rounded text-sm">
                                                    {row.clone()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>

                        <div>
                            <h3 class="text-lg font-medium text-gray-900 mb-2">"Desktop View"</h3>
                            <ScrollArea 
                                class="h-48 lg:h-64 w-full border rounded-md p-4".to_string()
                            >
                                <ScrollAreaViewport>
                                    <div class="grid grid-cols-2 gap-2">
                                        {table_data_6.iter().take(20).map(|row| {
                                            view! {
                                                <div class="p-2 bg-gray-50 rounded text-sm">
                                                    {row.clone()}
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </ScrollAreaViewport>
                            </ScrollArea>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

pub fn start_scroll_area_examples_fn() {
    mount_to_body(|| view! { <ScrollAreaExamples /> });
}
