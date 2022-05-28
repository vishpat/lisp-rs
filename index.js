const lisp = import('./pkg');
lisp
  .then(lisp => console.log('(+ 1 2)' + lisp.lisp_rs_eval("(+ 1 2)")))
  .catch(console.error);
