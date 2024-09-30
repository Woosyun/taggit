let tags = new Set<string>();

export function addTag(tag: string): boolean {
    if (!tag || tags.has(tag)) return false;
    tags.add(tag);
    return true;
}

export function deleteTag(tag: string): boolean {
    if (!tag || !tags.has(tag)) return false;
    tags.delete(tag);
    return true;
}

export function getTags(): string[] {
    return Array.from(tags);
}