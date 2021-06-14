import React from "react";

import {
  MDBRow,
  MDBCol,
  MDBContainer,
  MDBCard,
  MDBCardBody,
  MDBView,
  MDBMask,
} from "mdbreact";

const NotFound = () => {
  const navStyle = { height: "100%", width: "100%", paddingTop: "2rem" };

  return (
    <MDBView className="home-view">
      <MDBMask className="d-flex justify-content-center align-items-center gradient" />
      <MDBContainer
        style={navStyle}
        className="mt-5 d-flex justify-content-center align-items-center"
      >
        <MDBRow className="white-text">
          <MDBCol className="mb-4">
            <MDBCard color="white" text="black" id="classic-card">
              <MDBCardBody>404, Page Not Found</MDBCardBody>
            </MDBCard>
          </MDBCol>
        </MDBRow>
      </MDBContainer>
    </MDBView>
  );
};

export default NotFound;
