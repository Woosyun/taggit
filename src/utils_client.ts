export function roughByteSizeOfObject(object: any) {
    const objectList = new Set();
    const stack = [object];
    let bytes = 0;

    while (stack.length) {
        const value = stack.pop();

        // Prevent circular references by skipping already visited objects
        if (typeof value === 'object' && value !== null) {
            if (objectList.has(value)) {
                continue;
            }
            objectList.add(value);
        }

        switch (typeof value) {
            case 'boolean':
                bytes += 4;
                break;
            case 'number':
                bytes += 8;
                break;
            case 'string':
                bytes += value.length * 2; // strings are 2 bytes per character
                break;
            case 'object':
                if (Array.isArray(value)) {
                    stack.push(...value);
                } else if (value !== null) {
                    for (const key in value) {
                        stack.push(value[key]);
                    }
                }
                break;
        }
    }
    return bytes;
}