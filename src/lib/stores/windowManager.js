import { get, writable } from "svelte/store";

export const groups = writable({});

export function registerGroup(name) {
    let newGroups = {...get(groups), [name]: []};
    groups.set(newGroups);
    console.log(newGroups);
}

export function registerTab(group, title, window) {
    groups.update(groups => {
        groups[group].push({ title, window });
        return groups;
    });
}

export function setActiveTab(group, title) {
    groups.update(groups => {
        groups[group].forEach(tab => {
            if (tab.title === title) {
                tab.window.classList.remove('hidden');
            } else {
                tab.window.classList.add('hidden');
            }
        });

        return groups;
    });
}