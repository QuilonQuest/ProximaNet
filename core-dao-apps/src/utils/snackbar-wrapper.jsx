import React, {useEffect} from 'react'
import {useGlobalMutation} from './container'
import {MDBNotification} from "mdbreact";


export default function CustomizedSnackbar(props) {
  const mutationCtx = useGlobalMutation()
  const handleClose = (evt) => {
  }

  return (
    <>
      {props.toasts !== []  ? props.toasts.map((item, index) => (
         <MDBNotification
            //autohide={6000}
            bodyClassName="p-5 font-weight-bold white-text"
            className="stylish-color-dark"
            closeClassName="white-text"
            fade
            icon="bell"
            iconClassName="orange-text"
            message={item.message.toString()}
            show
            text=""
            title={item.variant}
            titleClassName="elegant-color-dark white-text"
            style={{
              position: "fixed",
              top: "60px",
              right: "10px",
              zIndex: 9999
            }}
          />
      )) : null}
    </>
  )
}
