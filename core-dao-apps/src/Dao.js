import React, {useState, useEffect} from 'react'
import Navbar from "./Navbar";
import Footer from "./Footer";
import useRouter from './utils/use-router'
import {useParams} from "react-router-dom";

import {
  MDBView, MDBMask, MDBContainer, MDBCard, MDBCardBody, MDBCardText,
  MDBCardTitle, MDBCardImage, MDBRow, MDBCol, MDBBtn, MDBCardHeader, MDBBadge,
  MDBNotification, MDBModal, MDBModalBody, MDBModalHeader, MDBModalFooter, MDBIcon,
  MDBInput, MDBBox
} from "mdbreact";
import {useGlobalState, useGlobalMutation} from './utils/container'
import {Contract} from "near-api-js";
import {Decimal} from 'decimal.js';
import Selector from "./Selector";
import {timestampToReadable, convertDuration, yoktoNear} from './utils/funcs'


const Proposal = (props) => {
  const [showModal, setShowModal] = useState(false);
  const [modalText, setModalText] = useState('');
  const [showSpinner, setShowSpinner] = useState(false);
  const stateCtx = useGlobalState();


  const vote = async (vote) => {
    try {
      setShowSpinner(true);
      await window.contract.vote({
        id: props.data.key,
        vote: vote,
      })
    } catch (e) {
      console.log(e);
      props.setShowError(e);
    } finally {
      setShowSpinner(false);
    }
  }

  const finalize = async () => {
    try {
      setShowSpinner(true);
      await window.contract.finalize({
        id: props.data.key,
      })
    } catch (e) {
      console.log(e);
      props.setShowError(e);
    } finally {
      setShowSpinner(false);
    }
  }

  const handleVoteYes = () => {
    vote('Yes').then().catch((e) => {
      console.log(e);
    });

  }

  const handleVoteNo = () => {
    vote('No').then().catch((e) => {
      console.log(e);
    });
  }

  const handleFinalize = () => {
    finalize().then().catch((e) => {
      console.log(e);
    });
  }

  return (
    <MDBCol className="col-12 col-sm-8 col-lg-6 mx-auto">
      <MDBCard className="mb-5">
        <MDBCardHeader className="text-center h4-responsive">
          {props.data.kind.type === 'ChangePurpose' ? "Change DAO Purpose: " + props.data.kind.purpose : null}
          {props.data.kind.type === 'NewCouncil' ? "New Council Member: " + props.data.target : null}
          {props.data.kind.type === 'RemoveCouncil' ? "Remove Council Member: " + props.data.target : null}
          {props.data.kind.type === 'ChangeVotePeriod' ? "Change Vote Period: " + `${props.data.kind.vote_period / 1e9 / 60 / 60 ^ 0}h ` + (props.data.kind.vote_period / 1e9 / 60 % 60).toFixed(0) + 'm' : null}

          {props.data.kind.type === "Payout" ?
            <div>
              <div className="float-left">
                Payout:
              </div>
              <div className="float-right font-weight-bold" style={{fontSize: 25}}>
                <span style={{fontSize: 22, marginRight: 2}}>Ⓝ</span>
                {(props.data.kind.amount / yoktoNear).toFixed(0).toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",")}
              </div>
            </div>
            : null}
          <div className="clearfix"/>

        </MDBCardHeader>
        <MDBCardBody>
          <div className="float-left">
            {(convertDuration(props.data.vote_period_end) < new Date() && props.data.status === 'Vote') ?
              <h4><MDBBadge color="danger">Expired</MDBBadge></h4>
              :
              <>
                {props.data.status === 'Fail' ?
                  <h4><MDBBadge color="danger">{props.data.status}</MDBBadge></h4>
                  :
                  null
                }
                {props.data.status === 'Success' ?
                  <h4><MDBBadge color="green">Passed / {props.data.status}</MDBBadge>{" "}<MDBIcon
                    className="amber-text"
                    icon="crown"/></h4>
                  :
                  null
                }
                {props.data.status === 'Vote' ?
                  <h4><MDBBadge color="green">Active / {props.data.status}</MDBBadge></h4>
                  :
                  null
                }
                {props.data.status === 'Reject' ?
                  <h4><MDBBadge color="danger">Passed / {props.data.status}</MDBBadge></h4>
                  :
                  null
                }
              </>
            }
          </div>
          <div className="float-right h4-responsive">#{props.data.key}</div>
          <div className="clearfix"/>
          <MDBCardText>
            <MDBBox className="h4-responsive black-text">{props.data.description}</MDBBox>
            <hr/>
            <div className="float-left text-muted h4-responsive">proposer</div>
            <div className="float-right h4-responsive">
              <a className="text-right" target="_blank"
                 href={stateCtx.config.network.explorerUrl + "/accounts/" + props.data.proposer.toLowerCase()}>{props.data.proposer.toLowerCase()}</a>
            </div>
            <br/>
            <div className="clearfix"/>
            <div className="float-left text-muted h4-responsive">target</div>
            <MDBBox className="float-right h4-responsive" style={{width: '80%'}}>
              <a className="text-right float-right" target="_blank" style={{wordBreak: "break-word"}}
                 href={stateCtx.config.network.explorerUrl + "/accounts/" + props.data.target.toLowerCase()}>{props.data.target.toLowerCase()}</a>
            </MDBBox>
            <div className="clearfix"/>
          </MDBCardText>
          {props.council.includes(window.walletConnection.getAccountId()) ?
            <div>
              <MDBBtn
                disabled={showSpinner || convertDuration(props.data.vote_period_end) < new Date() || props.data.status !== 'Vote'}
                onClick={handleVoteYes}
                color="green darken-1"
                className='h5-responsive'
                size="sm">Vote YES
                <MDBIcon icon='thumbs-up' size="2x" className='white-text ml-2'/>
                {showSpinner ?
                  <div className="spinner-border spinner-border-sm ml-2" role="status">
                    <span className="sr-only">Loading...</span>
                  </div>
                  : null}
              </MDBBtn>
              {(convertDuration(props.data.vote_period_end) < new Date() && props.data.status === 'Vote') ?
                <MDBBtn
                  disabled={showSpinner}
                  onClick={handleFinalize}
                  color="green"
                  className='h5-responsive float-right'
                  size="sm">Finalize
                  <MDBIcon icon="check-circle" size="2x" className='white-text ml-2'/>
                  {showSpinner ?
                    <div className="spinner-border spinner-border-sm ml-2" role="status">
                      <span className="sr-only">Loading...</span>
                    </div>
                    : null}
                </MDBBtn>
                : null}

              <MDBBtn
                disabled={showSpinner || convertDuration(props.data.vote_period_end) < new Date() || props.data.status !== 'Vote'}
                onClick={handleVoteNo}
                color="red"
                className='h5-responsive float-right'
                size="sm">Vote NO
                <MDBIcon icon='thumbs-down' size="2x" className='white-text ml-2'/>
                {showSpinner ?
                  <div className="spinner-border spinner-border-sm ml-2" role="status">
                    <span className="sr-only">Loading...</span>
                  </div>
                  : null}
              </MDBBtn>

            </div>
            : null
          }
        </MDBCardBody>
        <div className='rounded-bottom mdb-color lighten-3 text-center pt-3 pl-5 pr-5'>
          <ul className='list-unstyled list-inline font-small'>
            <li className='list-inline-item pr-2 white-text h4-responsive'>
              <MDBIcon far
                       icon='clock'/>{" "}{convertDuration(props.data.vote_period_end).toLocaleDateString()} {convertDuration(props.data.vote_period_end).toLocaleTimeString()}
            </li>
            <li className='list-inline-item pr-2'>
              <MDBIcon icon='thumbs-up' size="2x" className='lime-text mr-1'/>
              <span className="white-text h3-responsive">{props.data.vote_yes}</span>
            </li>
            <li className='list-inline-item pr-2'>
              <MDBIcon icon='thumbs-down' size="2x" className='amber-text mr-1'/>
              <span className="white-text h3-responsive">{props.data.vote_no}</span>
            </li>
          </ul>
        </div>
      </MDBCard>
      {/*<QuestionModal show={showModal} text={modalText} handleVoteYes={handleVoteYes}/>*/}
    </MDBCol>

  )


}

const Dao = () => {
  const [buttonDisabled, setButtonDisabled] = useState(true)
  const [showNotification, setShowNotification] = useState(false)
  const [numberProposals, setNumberProposals] = useState(0);
  const [proposals, setProposals] = useState(null);
  const [council, setCouncil] = useState([]);
  const [bond, setBond] = useState(0);
  const [daoVotePeriod, setDaoVotePeriod] = useState(0);
  const [daoPurpose, setDaoPurpose] = useState('');
  const [showError, setShowError] = useState(null);
  const [addProposalModal, setAddProposalModal] = useState(false);
  const [showSpinner, setShowSpinner] = useState(false);
  const [showCouncilChange, setShowCouncilChange] = useState(false);
  const [showPayout, setShowPayout] = useState(true);
  const [showChangePurpose, setShowChangePurpose] = useState(false);
  const [showVotePeriod, setShowVotePeriod] = useState(false);
  const [disableTarget, setDisableTarget] = useState(false);
  const routerCtx = useRouter()
  const stateCtx = useGlobalState()
  const mutationCtx = useGlobalMutation()
  const [selectDao, setSelectDao] = useState(false);

  let {dao} = useParams();
  if (dao !== undefined && stateCtx.config.contract === "") {
    mutationCtx.updateConfig({
      contract: dao,
    })
    /* TODO: re-do without reload */
    window.location.reload();
  }


  const [proposalKind, setProposalKind] = useState({
    value: "",
    valid: true,
    message: "",
  });
  const [proposalTarget, setProposalTarget] = useState({
    value: "",
    valid: true,
    message: "",
  });
  const [proposalDescription, setProposalDescription] = useState({
    value: "",
    valid: true,
    message: "",
  });
  const [proposalAmount, setProposalAmount] = useState({
    value: "",
    valid: true,
    message: "",
  });
  const [votePeriod, setVotePeriod] = useState({
    value: "",
    valid: true,
    message: "",
  });
  const [changePurpose, setChangePurpose] = useState({
    value: "",
    valid: true,
    message: "",
  });


  useEffect(
    () => {
      if (stateCtx.config.contract === "") {
        setSelectDao(true);
      } else {
        window.contract = new Contract(window.walletConnection.account(), stateCtx.config.contract, {
          viewMethods: ['get_council', 'get_bond', 'get_proposal', 'get_num_proposals', 'get_proposals', 'get_vote_period', 'get_purpose'],
          changeMethods: ['vote', 'add_proposal', 'finalize'],
        })
      }
    },
    [stateCtx.config.contract]
  )


  const toggleProposalModal = () => {
    setAddProposalModal(!addProposalModal);
  }

  const submitProposal = async (e) => {
    e.preventDefault();
    e.persist();

    let validateTarget = validateField("proposalTarget", proposalTarget.value);
    let validateDescription = validateField("proposalDescription", proposalDescription.value);
    let validateChangePurpose = validateField("changePurpose", changePurpose.value);
    let validateAmount = validateField("proposalAmount", proposalAmount.value);

    /*
    if (e.target.proposalKind.value === 'false') {
      e.target.proposalKind.className += " is-invalid";
      e.target.proposalKind.classList.remove("is-valid");
    } else {
      e.target.proposalKind.classList.remove("is-invalid");
      e.target.proposalKind.className += " is-valid";
    }
    */

    if (showChangePurpose) {
      if (!validateChangePurpose) {
        e.target.changePurpose.className += " is-invalid";
        e.target.changePurpose.classList.remove("is-valid");
      } else {
        e.target.changePurpose.classList.remove("is-invalid");
        e.target.changePurpose.className += " is-valid";
      }
    }

    if (!validateTarget) {
      e.target.proposalTarget.className += " is-invalid";
      e.target.proposalTarget.classList.remove("is-valid");
    } else {
      e.target.proposalTarget.classList.remove("is-invalid");
      e.target.proposalTarget.className += " is-valid";
    }

    if (!validateDescription) {
      e.target.proposalDescription.className += " is-invalid";
      e.target.proposalDescription.classList.remove("is-valid");
    } else {
      e.target.proposalDescription.classList.remove("is-invalid");
      e.target.proposalDescription.className += " is-valid";
    }


    if (showPayout) {
      if (!validateAmount) {
        e.target.proposalAmount.className += " is-invalid";
        e.target.proposalAmount.classList.remove("is-valid");
      } else {
        e.target.proposalAmount.classList.remove("is-invalid");
        e.target.proposalAmount.className += " is-valid";
      }
    }

    if (showPayout) {
      if (e.target.proposalKind.value !== 'false' && validateTarget && validateDescription && validateAmount) {
        try {
          setShowSpinner(true);
          const amount = new Decimal(e.target.proposalAmount.value);
          const amountYokto = amount.mul(yoktoNear).toFixed();

          await window.contract.add_proposal({
              proposal: {
                target: e.target.proposalTarget.value,
                description: e.target.proposalDescription.value,
                kind: {
                  type: e.target.proposalKind.value,
                  amount: amountYokto,
                }
              },
            },
            new Decimal("30000000000000").toString(), bond.toString(),
          )
        } catch (e) {
          console.log(e);
          setShowError(e);
          setAddProposalModal(false);
        } finally {
          setShowSpinner(false);
          setAddProposalModal(false);
        }
      }
    }

    if (showCouncilChange) {
      if (validateTarget && validateDescription) {
        try {
          setShowSpinner(true);
          await window.contract.add_proposal({
              proposal: {
                target: e.target.proposalTarget.value,
                description: e.target.proposalDescription.value,
                kind: {
                  type: e.target.proposalKind.value,
                }
              },
            },
            new Decimal("30000000000000").toString(), bond.toString(),
          )
        } catch (e) {
          console.log(e);
          setShowError(e);
          setAddProposalModal(false);
        } finally {
          setShowSpinner(false);
          setAddProposalModal(false);
        }
      }
    }
    if (showChangePurpose) {
      if (validateTarget && validateDescription && changePurpose) {
        try {
          setShowSpinner(true);
          await window.contract.add_proposal({
              proposal: {
                target: e.target.proposalTarget.value,
                description: e.target.proposalDescription.value,
                kind: {
                  type: 'ChangePurpose',
                  purpose: e.target.changePurpose.value,
                }
              },
            },
            new Decimal("30000000000000").toString(), bond.toString(),
          )
        } catch (e) {
          console.log(e);
          setShowError(e);
          setAddProposalModal(false);
        } finally {
          setShowSpinner(false);
          setAddProposalModal(false);
        }
      }
    }

    if (showVotePeriod) {
      if (validateTarget && validateDescription) {
        const votePeriod = new Decimal(e.target.votePeriod.value).mul('3.6e12');
        try {
          setShowSpinner(true);
          await window.contract.add_proposal({
              proposal: {
                target: e.target.proposalTarget.value,
                description: e.target.proposalDescription.value,
                kind: {
                  type: 'ChangeVotePeriod',
                  vote_period: votePeriod,
                }
              },
            },
            new Decimal("30000000000000").toString(), bond.toString(),
          )
        } catch (e) {
          console.log(e);
          setShowError(e);
          setAddProposalModal(false);
        } finally {
          setShowSpinner(false);
          setAddProposalModal(false);
        }
      }
    }


  }


  useEffect(
    () => {
      if (stateCtx.config.contract !== "") {
        window.contract.get_num_proposals()
          .then(number => {
            setNumberProposals(number);
            window.contract.get_proposals({from_index: 0, limit: number})
              .then(list => {
                const t = []
                list.map((item, key) => {
                  const t2 = {}
                  Object.assign(t2, {key: key}, item);
                  t.push(t2);
                })
                setProposals(t);
              });
          }).catch((e) => {
          console.log(e);
          setShowError(e);
        })
      }
    },
    [stateCtx.config.contract]
  )


  useEffect(
    () => {
      if (stateCtx.config.contract !== "") {
        window.contract.get_council()
          .then(r => {
            setCouncil(r);
          }).catch((e) => {
          console.log(e);
          setShowError(e);
        })
      }
    },
    [stateCtx.config.contract]
  )


  useEffect(
    () => {
      if (stateCtx.config.contract !== "") {
        window.contract.get_bond()
          .then(r => {
            setBond(r);
          }).catch((e) => {
          console.log(e);
          setShowError(e);
        })
      }
    },
    [stateCtx.config.contract]
  )

  useEffect(
    () => {
      if (stateCtx.config.contract !== "") {
        window.contract.get_purpose()
          .then(r => {
            setDaoPurpose(r)
          }).catch((e) => {
          console.log(e);
          setShowError(e);
        })

        window.contract.get_vote_period()
          .then(r => {
            setDaoVotePeriod(r)
          }).catch((e) => {
          console.log(e);
          setShowError(e);
        })
      }
    },
    [stateCtx.config.contract]
  )

  const handleDaoChange = () => {
    mutationCtx.updateConfig({
      contract: '',
    })
    routerCtx.history.push('/')
    /* TODO: Modal no-scroll workaround */
    window.location.reload();
  }

  const validateString = (field, name, showMessage) => {
    if (name && name.length >= 1) {
      return true;
    } else {
      showMessage(field + " > 1 chars", 'warning', field);
      return false;
    }
  }
  const validateLongString = (field, name, showMessage) => {
    if (name && name.length >= 3 && name.length <= 250) {
      return true;
    } else {
      showMessage(field + " > 3 and < 250 chars", 'warning', field);
      return false;
    }
  }

  const validateNumber = (field, name, showMessage) => {
    if (name && !isNaN(name) && name.length > 0) {
      return true;
    } else {
      showMessage("Please enter number", 'warning', field);
      return false;
    }
  }

  const validateField = (field, value) => {
    switch (field) {
      case "proposalKind":
        return value !== 'false';
      case "proposalTarget":
      case "changePurpose":
        return validateString(field, value, showMessage.bind(this));
      case "proposalDescription":
        return validateLongString(field, value, showMessage.bind(this));
      case "proposalAmount":
      case "votePeriod":
        return validateNumber(field, value, showMessage.bind(this));
    }
  };

  const changeSelectHandler = (event) => {
    if (event.target.value === "NewCouncil" || event.target.value === "RemoveCouncil") {
      setShowCouncilChange(true)
      setShowPayout(false)
      setShowChangePurpose(false)
      setShowVotePeriod(false)
      setDisableTarget(false)
      setProposalTarget({value: '', valid: true, message: ''});
    }

    if (event.target.value === "ChangeVotePeriod") {
      setShowVotePeriod(true)
      setShowPayout(false)
      setShowChangePurpose(false)
      setShowCouncilChange(false)
      setProposalTarget({value: window.walletConnection.getAccountId(), valid: true, message: ''});
      setDisableTarget(true)
    }

    if (event.target.value === "ChangePurpose") {
      setShowChangePurpose(true)
      setShowPayout(false)
      setShowVotePeriod(false)
      setShowCouncilChange(false)
      setProposalTarget({value: window.walletConnection.getAccountId(), valid: true, message: ''});
      setDisableTarget(true)
    }

    if (event.target.value === "Payout") {
      setShowPayout(true)
      setShowChangePurpose(false)
      setShowVotePeriod(false)
      setShowCouncilChange(false)
      setDisableTarget(false)
      setProposalTarget({value: '', valid: true, message: ''});
    }


    if (event.target.name === "proposalKind") {
      setProposalKind({value: event.target.value, valid: !!event.target.value});
    }

  };

  const changeHandler = (event) => {
    if (event.target.name === "proposalTarget") {
      setProposalTarget({value: event.target.value.toLowerCase(), valid: !!event.target.value});
    }
    if (event.target.name === "proposalDescription") {
      setProposalDescription({value: event.target.value, valid: !!event.target.value});
    }
    if (event.target.name === "proposalAmount") {
      setProposalAmount({value: event.target.value, valid: !!event.target.value});
    }
    if (event.target.name === "votePeriod") {
      setVotePeriod({value: event.target.value, valid: !!event.target.value});
    }
    if (event.target.name === "changePurpose") {
      setChangePurpose({value: event.target.value, valid: !!event.target.value});
    }

    if (!validateField(event.target.name, event.target.value)) {
      event.target.className = "form-control is-invalid";
    } else {
      event.target.className = "form-control is-valid";
    }
  };

  const showMessage = (message, type, field) => {
    message = message.trim();
    if (message) {
      switch (field) {
        case "proposalKind":
          setProposalKind({message: message});
          break;
        case "proposalTarget":
          setProposalTarget({message: message});
          break;
        case "proposalDescription":
          setProposalDescription({message: message});
          break;
        case "proposalAmount":
          setProposalAmount({message: message});
          break;
      }
    }
  };

  console.log(proposals);

  return (
    <MDBView className="w-100 h-100" style={{minHeight: "100vh"}}>
      <MDBMask className="d-flex justify-content-center grey lighten-2 align-items-center gradient"/>
      <Navbar/>
      <MDBContainer style={{minHeight: "100vh"}} className="">
        {stateCtx.config.contract ?
          <>
            <MDBRow>
              <MDBCol className="col-12 p-3 mx-auto">
                <MDBCard>
                  <MDBCardBody>
                    <MDBRow>
                      <MDBCol>
                        <MDBCard className="p-0 m-2">
                          <MDBCardHeader className="h4-responsive">Council:</MDBCardHeader>
                          <MDBCardBody className="p-4">
                            {council ? council.map(item => (
                              <li>{item}</li>
                            )) : ''}
                          </MDBCardBody>
                        </MDBCard>
                      </MDBCol>
                      <MDBCol className="col-12 col-md-6">
                        <MDBCard className="p-0 m-2">
                          <MDBCardHeader className="h5-responsive">
                            <MDBRow>
                              <MDBCol>
                                Properties:
                              </MDBCol>
                              <MDBCol className="">
                                <MDBBox className="text-right">
                                  <MDBBtn size="sm" onClick={handleDaoChange} color="secondary">Change DAO</MDBBtn>
                                </MDBBox>
                              </MDBCol>
                            </MDBRow>
                          </MDBCardHeader>
                          <MDBCardBody className="p-2">


                            <ul>
                              <li>Network: <a target="_blank"
                                              href={stateCtx.config.network.explorerUrl}>{stateCtx.config.network.networkId}</a>
                              </li>
                              <li>DAO: {stateCtx.config.contract}</li>
                              <li>Bond: {bond ? (new Decimal(bond).div(yoktoNear)).toString() : ''} Ⓝ</li>
                              <li>Purpose: {daoPurpose}</li>
                              <li>Vote Period: {daoVotePeriod ? timestampToReadable(daoVotePeriod) : ''}</li>
                            </ul>
                          </MDBCardBody>
                        </MDBCard>
                      </MDBCol>
                    </MDBRow>
                    {window.walletConnection.getAccountId() ?
                      <MDBRow className="mx-auto p-2">
                        <MDBCol className="text-center">
                          <MDBBtn size="sm" onClick={toggleProposalModal} color="unique">ADD NEW PROPOSAL</MDBBtn>
                        </MDBCol>
                      </MDBRow>
                      : null}
                  </MDBCardBody>
                </MDBCard>
              </MDBCol>
            </MDBRow>

            <MDBRow className="">
              {numberProposals > 0 && proposals !== null ?
                proposals.sort((a, b) => b.key >= a.key ? 1 : -1).map((item, key) => (
                  <Proposal data={item} key={key} id={key} council={council} setShowError={setShowError}/>
                ))
                : null}
            </MDBRow>
            {showError !== null ?
              <MDBNotification
                //autohide={6000}
                bodyClassName="p-5 font-weight-bold white-text"
                className="stylish-color-dark"
                closeClassName="white-text"
                fade
                icon="bell"
                iconClassName="orange-text"
                message={showError.toString().trim()}
                show
                text=""
                title=""
                titleClassName="elegant-color-dark white-text"
                style={{
                  position: "fixed",
                  top: "60px",
                  right: "10px",
                  zIndex: 9999
                }}
              />
              : null
            }
            <MDBModal isOpen={addProposalModal} toggle={toggleProposalModal} centered position="center" size="lg">
              <MDBModalHeader className="text-center" titleClass="w-100 font-weight-bold" toggle={toggleProposalModal}>
                Add New Proposal
              </MDBModalHeader>
              <form className="needs-validation mx-3 grey-text"
                    name="proposal"
                    noValidate
                    method="post"
                    onSubmit={submitProposal}
              >
                <MDBModalBody>
                  <div className="pl-3 pr-3 mb-2">
                    <select onChange={changeSelectHandler} name="proposalKind" required
                            className="browser-default custom-select">
                      {/*<option value="false">Choose proposal type</option>*/}
                      <option value="Payout">Payout</option>
                      <option value="NewCouncil">New Council</option>
                      <option value="RemoveCouncil">Remove Council</option>
                      <option value="ChangeVotePeriod">Change Vote Period</option>
                      <option value="ChangePurpose">Change Purpose</option>
                    </select>
                  </div>
                  <MDBInput disabled={disableTarget} name="proposalTarget" value={proposalTarget.value}
                            onChange={changeHandler} label="Target"
                            required group>
                    <div className="invalid-feedback">
                      {proposalTarget.message}
                    </div>
                  </MDBInput>
                  <MDBInput name="proposalDescription" value={proposalDescription.value} onChange={changeHandler}
                            required label="Job/proposal description (max 250 chars)" group>
                    <div className="invalid-feedback">
                      {proposalDescription.message}
                    </div>
                  </MDBInput>
                  {showPayout ?
                    <MDBInput value={proposalAmount.value} name="proposalAmount" onChange={changeHandler} required
                              label="Payout in NEAR" group>
                      <div className="invalid-feedback">
                        {proposalAmount.message}
                      </div>
                    </MDBInput>
                    : null}
                  {showVotePeriod ?
                    <MDBInput value={votePeriod.value} name="votePeriod" onChange={changeHandler} required
                              label="New Vote Period (in hours)" group>
                      <div className="invalid-feedback">
                        {votePeriod.message}
                      </div>
                    </MDBInput>
                    : null}
                  {showChangePurpose ?
                    <MDBInput value={changePurpose.value} name="changePurpose" onChange={changeHandler} required
                              label="Enter New Purpose" group>
                      <div className="invalid-feedback">
                        {changePurpose.message}
                      </div>
                    </MDBInput>
                    : null}
                  <MDBInput
                    value={bond ? "Bond: " + (new Decimal(bond.toString()).div(yoktoNear).toFixed(0)) + " NEAR (amount to pay now)" : null}
                    name="bondAmount" disabled group/>
                </MDBModalBody>
                <MDBModalFooter className="justify-content-center">
                  <MDBBox className="font-small">For more details on your proposal or to start the discussion, please post in <a
                    href="https://gov.near.org/c/community/10"
                    target="_blank">https://gov.near.org/c/community/10</a>
                  </MDBBox>
                  <MDBBtn color="unique" type="submit">
                    Submit
                    {showSpinner ?
                      <div className="spinner-border spinner-border-sm ml-2" role="status">
                        <span className="sr-only">Loading...</span>
                      </div>
                      : null}
                  </MDBBtn>
                </MDBModalFooter>
              </form>
            </MDBModal>
          </>
          : null}
        {selectDao ?
          <Selector/>
          : null
        }
      </MDBContainer>
      <Footer/>
    </MDBView>

  )
}

export default Dao;

