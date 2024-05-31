import { get, writable } from "svelte/store";

export const groups = writable({});

export function registerGroup(name) {
    let newGroups = {...get(groups), [name]: {}};
    groups.set(newGroups);
    console.log(newGroups);
}

export function registerTab(group, title, window) {
    groups.update(groups => {
        groups[group][title] = window;
        window.classList.add('hidden');
        return groups;
    });
}

export function setActiveTab(group, title) {
    groups.update(groups => {
        for (let tab in groups[group]) {
            groups[group][tab].classList.add('hidden');
        }

        groups[group][title].classList.remove('hidden');
        return groups;
    });
}