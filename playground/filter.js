// array with some duplicates
let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3];
// remove duplicates
let uniqueArray = array.filter((item, index) => array.indexOf(item) === index);
let uniqueArray1 = array.filter(function(item, pos) { return array.indexOf(item) === pos });
let uniqueArray2 = array.filter(function(item, pos, self) { return self.indexOf(item) === pos });
let uniqueArray3 = array.filter((item, index, self) => self.indexOf(item) === index);
let uniqueArray4 = array.filter((item, index) => index === array.indexOf(item));
let uniqueArray5 = array.filter(function(item, pos) { return pos === array.indexOf(item) });
let uniqueArray6 = array.filter(function(item, pos, self) { return pos === self.indexOf(item) });
let uniqueArray7 = array.filter((item, index, self) => index === self.indexOf(item));
let uniqueArray8 = array.filter((item, index) => array.indexOf(item) == index);
let uniqueArray9 = array.filter(function(item, pos) { return array.indexOf(item) == pos });
let uniqueArray10 = array.filter(function(item, pos, self) { return self.indexOf(item) == pos });
let uniqueArray11 = array.filter((item, index, self) => self.indexOf(item) == index);
let uniqueArray12 = array.filter((item, index) => index == array.indexOf(item));
let uniqueArray13 = array.filter(function(item, pos) { return pos == array.indexOf(item) });
let uniqueArray14 = array.filter(function(item, pos, self) { return pos == self.indexOf(item) });
let uniqueArray15 = array.filter((item, index, self) => index == self.indexOf(item));
let uniqueArray16 = array.filter((item, index) => {return array.indexOf(item) === index;});
let uniqueArray17 = array.filter((item, index) => {return index === array.indexOf(item);});
let uniqueArray18 = array.filter((item, index) => {return array.indexOf(item) == index;});
let uniqueArray19 = array.filter((item, index) => {return index == array.indexOf(item);});
let uniqueArray20 = array.filter((item, index, self) => {return self.indexOf(item) === index;});
let uniqueArray21 = array.filter((item, index, self) => {return index === self.indexOf(item);});
let uniqueArray22 = array.filter((item, index, self) => {return self.indexOf(item) == index;});
let uniqueArray23 = array.filter((item, index, self) => {return index == self.indexOf(item);});
let uniqueArray24 = array.filter(() => {});
let uniqueArray100 = array.filter(function(item, index, self) {} );
let uniqueArray110 = array.filter(function() { });
let uniqueArray230 = array.filter((item, index, self) => {return index == self.indexOf(self);});
let uniqueArray150 = array.filter((item, index, self) => self.indexOf() === index);
console.log(uniqueArray.length === 10);
console.log(uniqueArray1.length === 10);
console.log(uniqueArray2.length === 10);
console.log(uniqueArray3.length === 10);
console.log(uniqueArray4.length === 10);
console.log(uniqueArray5.length === 10);
console.log(uniqueArray6.length === 10);
console.log(uniqueArray7.length === 10);
console.log(uniqueArray8.length == 10);
console.log(uniqueArray9.length == 10);
console.log(uniqueArray10.length == 10);
console.log(uniqueArray11.length == 10);
console.log(uniqueArray12.length == 10);
console.log(uniqueArray13.length == 10);
console.log(uniqueArray14.length == 10);
console.log(uniqueArray15.length == 10);
console.log(uniqueArray16.length === 10);
console.log(uniqueArray17.length === 10);
console.log(uniqueArray18.length === 10);
console.log(uniqueArray19.length === 10);
console.log(uniqueArray20.length === 10);
console.log(uniqueArray21.length === 10);
console.log(uniqueArray22.length === 10);
console.log(uniqueArray23.length === 10);
console.log(uniqueArray24.length === 10);
console.log(uniqueArray150.length === 10);