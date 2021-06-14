export const yoktoNear = 1000000000000000000000000;

export const timestampToReadable = (timestamp) => {
  let seconds = Math.floor(timestamp / 1e9);
  let minutes = Math.floor(seconds / 60);
  let hours = Math.floor(minutes / 60);
  let days = Math.floor(hours / 24);
  hours = hours - (days * 24);
  minutes = minutes - (days * 24 * 60) - (hours * 60);
  seconds = seconds - (days * 24 * 60 * 60) - (hours * 60 * 60) - (minutes * 60);

  if (days === 0) {
    if (seconds === 0) {
      return hours + 'h ';
    } else {
      return hours + 'h ' + seconds + 's';
    }
  }

  if (hours === 0 && seconds === 0) {
    return days + 'd';
  }

  if (seconds === 0) {
    return days + 'd ' + hours + 'h ';
  }

  return days + 'd ' + hours + 'h ' + seconds + 's';
}

export const convertDuration = (duration) => {
  let utcSeconds = duration / 1e9;
  let epoch = new Date(0);
  epoch.setUTCSeconds(utcSeconds);
  return epoch;
}




