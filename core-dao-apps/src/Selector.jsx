import React, {useEffect, useState} from 'react'
import {Contract} from "near-api-js";

import {
  MDBBox,
  MDBBtn,
  MDBCard,
  MDBCardBody,
  MDBCardHeader,
  MDBCollapse,
  MDBCollapseHeader,
  MDBListGroup,
  MDBListGroupItem,
  MDBModal,
  MDBModalBody,
  MDBModalFooter,
  MDBModalHeader
} from "mdbreact";
import {useGlobalMutation, useGlobalState} from './utils/container'
import useRouter from "./utils/use-router";
import {Decimal} from "decimal.js";
import {timestampToReadable, yoktoNear} from './utils/funcs'


const DaoInfo = (props) => {
  const contractId = props.item;
  const [council, setCouncil] = useState([]);
  const [bond, setBond] = useState(null);
  const [votePeriod, setVotePeriod] = useState(null);
  const [gracePeriod, setGracePeriod] = useState(null);
  const [purpose, setPurpose] = useState(null);
  const [collapseState, setCollapseState] = useState(false);

  const contract = new Contract(window.walletConnection.account(), contractId, {
    viewMethods: ['get_council', 'get_bond', 'get_num_proposals', 'get_purpose', 'get_vote_period'],
    changeMethods: [],
  })


  useEffect(
    () => {
      contract.get_bond().then((data) => {
        setBond(data);
      });
    }, [])


  useEffect(
    () => {
      contract.get_vote_period().then((data) => {
        setVotePeriod(data);
      });
    }, [])

  useEffect(
    () => {
      contract.get_purpose().then((data) => {
        setPurpose(data);
      });
    }, [])


  const toggleCollapse = () => {
    contract.get_council().then((data) => {
      setCouncil(data);
    });
    setCollapseState(!collapseState);
  }

  return (
    <div className="text-left">
      <MDBBox>
        <span className="text-muted">Bond:</span> {bond !== null ? (new Decimal(bond.toString()).div(yoktoNear)).toString() : ''}NEAR;{" "}
        <span className="text-muted">Vote Period:</span> {votePeriod ? timestampToReadable(votePeriod): ''};{" "}
        <span className="text-muted">Purpose:</span> <b>{purpose}</b>
      </MDBBox>
      <MDBCollapseHeader className="text-right p-2 m-0 font-small white" onClick={toggleCollapse}>
        view council{" "}
        <i className={collapseState ? "fa fa-angle-down rotate-icon" : "fa fa-angle-down"}/>
      </MDBCollapseHeader>
      <MDBCollapse className="p-0 m-0 mb-2" isOpen={collapseState}>
        <MDBListGroup className="text-left">
          {council.map((item, key) => <MDBListGroupItem className="p-1" key={key}>{item}</MDBListGroupItem>)}
        </MDBListGroup>
      </MDBCollapse>
    </div>
  );
}

const Selector = () => {
  const routerCtx = useRouter()
  const stateCtx = useGlobalState()
  const mutationCtx = useGlobalMutation()
  const [daoList, setDaoList] = useState([]);
  //const [daoInfo, setDaoInfo] = useState({});
  const [showModal, setShowModal] = useState(false);

  useEffect(
    () => {
      if (stateCtx.config.contract === "") {
        setShowModal(true);
      }

    },
    [stateCtx.config.contract]
  );

  useEffect(
    () => {
      window.factoryContract.get_dao_list()
        .then(r => {
          setDaoList(r);
          setShowModal(true);
        }).catch((e) => {
        console.log(e);
        mutationCtx.toastError(e);
        setShowModal(false);
      })
    },
    []
  )

  const handleSelect = async (e) => {
    e.preventDefault();

    mutationCtx.updateConfig({
      contract: e.target.name,
      bond: '',
      purpose: '',
      votePeriod: '',
    });
    setShowModal(false);
    routerCtx.history.push('/' + e.target.name);
    return false;
  }

  const toggleModal = () => {
    setShowModal(!showModal);
  }

  return (
    <MDBModal modalStyle="info" className="dark text-white default-color-dark"  size="lg"
              isOpen={showModal} overflowScroll={true}>
      <MDBModalHeader className="text-center" titleClass="w-100" tag="p">
        Please select or change DAO
      </MDBModalHeader>
      <MDBModalBody className="text-center">
        <MDBCard className="">
          <MDBCardBody className="p-4">
            {daoList ? daoList.map((item, key) => (
              <MDBCard className="m-2" key={key}>
                <MDBCardHeader color="white-text unique-color" className="h5-responsive">{item}</MDBCardHeader>
                <MDBCardBody>
                  <DaoInfo item={item}/>
                </MDBCardBody>
                <div className="text-right">
                <MDBBtn name={item} onClick={handleSelect} color="secondary" size="sm"
                        className="col-md-3">SELECT</MDBBtn>
                </div>
              </MDBCard>
            )) : ''}
          </MDBCardBody>
        </MDBCard> </MDBModalBody>
      <MDBModalFooter className="justify-content-center">
      </MDBModalFooter>
    </MDBModal>
  )
}

export default Selector;
