import { onDestroy } from 'svelte'
import { writable } from 'svelte/store';
import { api } from '../api'

const last_changed = writable(0);
let subscriptions = 0

/**
 * This must be called at the root of a component, and will
 * subscribe to changes, unsubscribing on component unmount.
 *
 * This fires immediately and then after any changes.
 */
export const on_last_changed = (fn: () => void) => {
    subscriptions++
    const unsub = last_changed.subscribe(fn)
    onDestroy(() => {
        unsub()
        subscriptions--
    })
}

// Periodically check for changes, and increment the
// last_changed counter if any happen:
let last_date: undefined | string = undefined
setInterval(() => {
    if (!subscriptions) return
    api.last_changed().then(({ date: new_date }) => {
        if (!last_date) {
            last_date = new_date
        }
        if (last_date != new_date) {
            last_changed.update(n => n+1)
        }
    }).catch((e) => {
        console.error(`Failed to get last_changed`)
    })
}, 1000)
