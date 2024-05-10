import { Component } from 'solid-js';

import './navbar.css';

const Navbar: Component = () => {
    return (
        <header class='navbar'>
            <div class='navbarItem'>
                <a href='/' class="navbarlogo" style={``}><h1>Wittle Workshop</h1></a>
            </div>
            <div class='navbarItem'>
                <a href='/'>Plans</a>
            </div>
            <div class='navbarItem'>
                 <a href='/managedocker'>Manage Docker</a>
            </div>
            <div class='navbarItem'>
                <a href='/manageservices'>Manage Services</a>
            </div>
        </header>
    );
};

export default Navbar;
