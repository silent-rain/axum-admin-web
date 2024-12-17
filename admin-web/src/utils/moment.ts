function dateState() {
  const date = new Date();
  if (date.getHours() >= 6 && date.getHours() < 12) {
    return '上午';
  } else if (date.getHours() >= 12 && date.getHours() < 18) {
    return '下午';
  } else {
    return '晚上';
  }
}

function setDayNight() {
  const date = new Date();
  if (date.getHours() >= 6 && date.getHours() < 18) {
    return '白天';
  } else {
    return '晚上';
  }
}
export { dateState, setDayNight };
