import './styles.css';
import { search } from './utils_server';

let tagBar: any;
let noteContainer: any;
let itemContainer: any;

window.addEventListener('DOMContentLoaded', () => {
    tagBar = document.querySelector('#tagbar');
    noteContainer = document.querySelector('#note-container');
    itemContainer = document.querySelector('#item-container');
    
    document.querySelector('#search-form')?.addEventListener('submit', async (e: any) => {
        e.preventDefault();
        
        const searchInput = (document.querySelector('#search-input') as HTMLInputElement);
        if (searchInput && !tags.has(searchInput.value)) {
            tags.add(searchInput.value);
            renderTagbar();
        }
        search();
        searchInput.value = '';
    });
});

let tags = new Set<string>();
function renderTagbar() {
    tagBar.innerHTML = '';
    tags.forEach(tag => {
        const tagElement = document.createElement('div');
        tagElement.classList.add('badge');
        tagElement.innerHTML = tag;
        tagElement.addEventListener('click', () => {
            if (!tags.has(tag)) return;

            tags.delete(tag);
            renderTagbar();
            search();
        });
        tagBar.appendChild(tagElement);
    });
}