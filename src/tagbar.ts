import { search } from "./main";

let tags = new Set<string>();
let tagbar: HTMLDivElement

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

export function initTagbar() {
    tagbar = document.querySelector('#tagbar') as HTMLDivElement;
    if (!tagbar) {
        console.error('missing tagbar');
        return;
    }
}

export function renderTagbar() {
    tagbar.innerHTML = '';
    getTags().forEach(tag => {
        const tagElement = document.createElement('div');
        tagElement.classList.add('badge');
        tagElement.innerHTML = tag;
        tagElement.addEventListener('click', () => {
            if (!deleteTag(tag)) return;
            renderTagbar();
            search();
        });
        tagbar.appendChild(tagElement);
    });
}