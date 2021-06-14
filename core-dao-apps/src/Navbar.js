import React, {Component, useState} from "react";
import {
  MDBNavbar, MDBNavbarBrand, MDBNavbarNav, MDBNavItem, MDBNavLink, MDBNavbarToggler, MDBCollapse, MDBDropdown,
  MDBDropdownToggle, MDBDropdownMenu, MDBDropdownItem, MDBIcon, MDBBtn
} from "mdbreact";
import {BrowserRouter as Router} from 'react-router-dom';
import {login, logout} from './utils'
import {useGlobalState, useGlobalMutation} from './utils/container'


const Navbar = () => {
  const [isOpen, setIsOpen] = useState(false);
  const stateCtx = useGlobalState()
  const mutationCtx = useGlobalMutation()

  const toggleCollapse = () => {
    setIsOpen(!isOpen);
  }

  return (
    <Router>
      <MDBNavbar color="unique-color" dark expand="md">
        <MDBNavbarBrand>
          <MDBNavLink className="white-text mr-2" to="/">
            <img className="d-inline-block" style={{filter: "brightness(0) invert(1)"}} height="30" src="https://gov.near.org/uploads/default/original/1X/7aa6fc28cbccdc2242717e8fe4c756829d90aaec.png" />
            sputnikDAO
          </MDBNavLink>
        </MDBNavbarBrand>
        <MDBNavbarToggler onClick={toggleCollapse}/>
        <MDBCollapse id="navbarCollapse3" isOpen={isOpen} navbar>
          <MDBNavbarNav className="white-text" left>
            <MDBNavItem className="">
              <MDBNavbarBrand>{stateCtx.config.contract}</MDBNavbarBrand>
            </MDBNavItem>
          </MDBNavbarNav>
          <MDBNavbarNav right>
            {!window.walletConnection.isSignedIn() ?
              <MDBNavItem active>
                <MDBNavLink to="#" onClick={login}>Sign In</MDBNavLink>
              </MDBNavItem>
              :
              <>
                <MDBNavItem active>
                  <MDBNavbarBrand>{window.accountId}</MDBNavbarBrand>
                </MDBNavItem>
                <MDBNavItem active>
                  <MDBNavLink to="#" onClick={logout}>Sign Out</MDBNavLink>
                </MDBNavItem>
              </>
            }
          </MDBNavbarNav>
        </MDBCollapse>
      </MDBNavbar>
    </Router>
  );

}

export default Navbar;
