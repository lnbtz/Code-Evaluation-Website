// array with some duplicates
let array = [];
for (let i = 0; i < 100000; i++) {
    // add random numbers
    array.push(Math.floor(Math.random() * 100000));
}

let set = [...new Set(array)];
// remove duplicates
console.time('uniqueArray');
let uniqueArray = array.filter((item, index) => array.indexOf(item) === index);
console.log(set.length == uniqueArray.length);
console.timeEnd('uniqueArray');

console.time('uniqueArray1');
let uniqueArray1 = array.filter(function(item, pos) { return array.indexOf(item) === pos });
console.log(set.length == uniqueArray1.length);
console.timeEnd('uniqueArray1');

console.time('uniqueArray2');
let uniqueArray2 = array.filter(function(item, pos, self) { return self.indexOf(item) === pos });
console.log(set.length == uniqueArray2.length);
console.timeEnd('uniqueArray2');

console.time('uniqueArray3');
let uniqueArray3 = array.filter((item, index, self) => self.indexOf(item) === index);
console.log(set.length == uniqueArray3.length);
console.timeEnd('uniqueArray3');

console.time('uniqueArray4');
let uniqueArray4 = array.filter((item, index) => index === array.indexOf(item));
console.log(set.length == uniqueArray4.length);
console.timeEnd('uniqueArray4');

console.time('uniqueArray5');
let uniqueArray5 = array.filter(function(item, pos) { return pos === array.indexOf(item) });
console.log(set.length == uniqueArray5.length);
console.timeEnd('uniqueArray5');

console.time('uniqueArray6');
let uniqueArray6 = array.filter(function(item, pos, self) { return pos === self.indexOf(item) });
console.log(set.length == uniqueArray6.length);
console.timeEnd('uniqueArray6');

console.time('uniqueArray7');
let uniqueArray7 = array.filter((item, index, self) => index === self.indexOf(item));
console.log(set.length == uniqueArray7.length);
console.timeEnd('uniqueArray7');

console.time('uniqueArray8');
let uniqueArray8 = array.filter((item, index) => array.indexOf(item) == index);
console.log(set.length == uniqueArray8.length);
console.timeEnd('uniqueArray8');

console.time('uniqueArray9');
let uniqueArray9 = array.filter(function(item, pos) { return array.indexOf(item) == pos });
console.log(set.length == uniqueArray9.length);
console.timeEnd('uniqueArray9');

console.time('uniqueArray10');
let uniqueArray10 = array.filter(function(item, pos, self) { return self.indexOf(item) == pos });
console.log(set.length == uniqueArray10.length);
console.timeEnd('uniqueArray10');

console.time('uniqueArray11');
let uniqueArray11 = array.filter((item, index, self) => self.indexOf(item) == index);
console.log(set.length == uniqueArray11.length);
console.timeEnd('uniqueArray11');

console.time('uniqueArray12');
let uniqueArray12 = array.filter((item, index) => index == array.indexOf(item));
console.log(set.length == uniqueArray12.length);
console.timeEnd('uniqueArray12');

console.time('uniqueArray13');
let uniqueArray13 = array.filter(function(item, pos) { return pos == array.indexOf(item) });
console.log(set.length == uniqueArray13.length);
console.timeEnd('uniqueArray13');

console.time('uniqueArray14');
let uniqueArray14 = array.filter(function(item, pos, self) { return pos == self.indexOf(item) });
console.log(set.length == uniqueArray14.length);
console.timeEnd('uniqueArray14');

console.time('uniqueArray15');
let uniqueArray15 = array.filter((item, index, self) => index == self.indexOf(item));
console.log(set.length == uniqueArray15.length);
console.timeEnd('uniqueArray15');

console.time('uniqueArray16');
let uniqueArray16 = array.filter((item, index) => {return array.indexOf(item) === index;});
console.log(set.length == uniqueArray16.length);
console.timeEnd('uniqueArray16');

console.time('uniqueArray17');
let uniqueArray17 = array.filter((item, index) => {return index === array.indexOf(item);});
console.log(set.length == uniqueArray17.length);
console.timeEnd('uniqueArray17');

console.time('uniqueArray18');
let uniqueArray18 = array.filter((item, index) => {return array.indexOf(item) == index;});
console.log(set.length == uniqueArray18.length);
console.timeEnd('uniqueArray18');

console.time('uniqueArray19');
let uniqueArray19 = array.filter((item, index) => {return index == array.indexOf(item);});
console.log(set.length == uniqueArray19.length);
console.timeEnd('uniqueArray19');

console.time('uniqueArray20');
let uniqueArray20 = array.filter((item, index, self) => {return self.indexOf(item) === index;});
console.log(set.length == uniqueArray20.length);
console.timeEnd('uniqueArray20');

console.time('uniqueArray21');
let uniqueArray21 = array.filter((item, index, self) => {return index === self.indexOf(item);});
console.log(set.length == uniqueArray21.length);
console.timeEnd('uniqueArray21');

console.time('uniqueArray22');
let uniqueArray22 = array.filter((item, index, self) => {return self.indexOf(item) == index;});
console.log(set.length == uniqueArray22.length);
console.timeEnd('uniqueArray22');

console.time('uniqueArray23');
let uniqueArray23 = array.filter((item, index, self) => {return index == self.indexOf(item);});
console.log(set.length == uniqueArray23.length);
console.timeEnd('uniqueArray23');

console.time('uniqueArray24');
let uniqueArray24 = array.filter(() => {});
console.log(set.length == uniqueArray24.length);
console.timeEnd('uniqueArray24');

console.time('uniqueArray100');
let uniqueArray100 = array.filter(function(item, index, self) {});
console.log(set.length == uniqueArray100.length);
console.timeEnd('uniqueArray100');

console.time('uniqueArray110');
let uniqueArray110 = array.filter(function() {});
console.log(set.length == uniqueArray110.length);
console.timeEnd('uniqueArray110');

console.time('uniqueArray230');
let uniqueArray230 = array.filter((item, index, self) => {return index == self.indexOf(self);});
console.log(set.length == uniqueArray230.length);
console.timeEnd('uniqueArray230');

console.time('uniqueArray150');
let uniqueArray150 = array.filter((item, index, self) => self.indexOf() === index);
console.log(set.length == uniqueArray150.length);
console.timeEnd('uniqueArray150');

let startingIndex = 1;
console.time('uniqueArray250');
let uniqueArray250 = array.filter((item, index, self) => self.indexOf(item, startingIndex) === index);
console.log(set.length == uniqueArray250.length);
console.timeEnd('uniqueArray250');