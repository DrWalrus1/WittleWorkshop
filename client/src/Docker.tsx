import type { Component } from 'solid-js';

import logo from './logo.svg';
import Homestyles from './Home.module.css';
import Dockerstyles from './Docker.module.css';

const Docker: Component = () => {
  return (
    <div class={Homestyles.App}>
      <header class={Homestyles.header}>
        <img src={logo} class={Homestyles.logo} alt="logo" />
        <p>
          Manage your Docker!
        </p>
        <div class={Dockerstyles.containerGrid}>
          {Array.from({ length: 5 }, () => <DockerContainerView />)}
        </div>
      </header>
    </div>
  );
};

function DockerContainerView() {
  return (
    <div class={Dockerstyles.containerView}>
      <h1>Container Name</h1>
      <div>
        <p>This is where the container details should be</p>
        <h4>Status: Running</h4>
        <button class={Dockerstyles.containerViewStartButton}>Stop Container</button>
      </div>
    </div>
  );
}

export default Docker;
