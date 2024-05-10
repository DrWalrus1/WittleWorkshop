import type { Component } from 'solid-js';

import styles from './Home.module.css';

const Services: Component = () => {
    return (
        <div class={styles.App}>
            <header class={styles.header}>
                <p>
                    Manage your Services!
                </p>
            </header>
        </div>
    );
};

export default Services;