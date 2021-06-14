import React, { useEffect, useState } from "react";
import { MDBCol, MDBContainer, MDBModal, MDBModalBody, MDBRow } from "mdbreact";

const Loading = () => {
  const [show, setShow] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setShow(true);
    }, 500);
    return () => clearTimeout(timer);
  }, []);

  return (
    <>
      {show ? (
        <MDBContainer fluid>
          <MDBRow>
            <MDBCol size="10" sm="8" lg="6">
              <MDBModal isOpen={true} toggle={() => {}} position="top">
                <MDBModalBody className="text-center">Loading from chain</MDBModalBody>
              </MDBModal>
            </MDBCol>
          </MDBRow>
        </MDBContainer>
      ) : null}
    </>
  );
};

export default Loading;
