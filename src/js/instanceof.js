let isNil = (x) => x === undefined || x === null;

var checkIfInstanceOf = function(obj, classFunction) {
  if (isNil(obj)) return false;
  if (typeof classFunction !== 'function') return false;
  if (obj instanceof classFunction) return true;
  if (obj.constructor === classFunction) return true;
  return obj.constructor instanceof classFunction;
};