use error::*;
use std::iter::repeat;

pub fn vector_add(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Vec<u32> {
    // determine the largest of the two numbers, and return a tuple
    // of reversed iterators with that larger number taking precedence
    let (larger, smaller) = 
        if rhs.len() > lhs.len() {
            (rhs.iter().rev(), lhs.iter().rev())
        } else {
            (lhs.iter().rev(), rhs.iter().rev())
        };
    
    let mut carry = 0;
    let mut result: Vec<u32>  = Vec::new();

    // if the smaller number does not contain enough digits for the zip,
    // it will instead be filled with `None` values to denote such
    let zipped_iters = larger.zip(smaller.map(|v| Some(v)).chain(repeat(None)));
    
    for x in zipped_iters {
        // return the result of adding the two operands together, making
        // sure to take into account that `None` may appear meaning the
        // second number is out of digits
        let mut idx_add = match x.1 {
            Some(y) => { x.0 + y + carry},
            None    => { x.0 + carry}
        };

        // keep track of a carry which is applied to each addition result
        carry = idx_add / 10;
        idx_add = idx_add % 10;

        result.push(idx_add);
    }

    // push the extra carry if it exists(e.g. is not zero)
    if carry != 0 {
        result.push(carry);
    }

    result.into_iter().rev().collect::<Vec<_>>()
}

pub fn vector_sub(lhs: &Vec<u32>, rhs: &Vec<u32>) -> Result<Vec<u32>, Error> {
    // easy checks to see if the result will be a negative number
    if lhs.len() < rhs.len() || (lhs.len() == rhs.len() && 
       lhs[0] < rhs[0]) {
        return Err(Error::new(
                ErrorType::UnsignedOverflow,
                "Subtraction of unsigned numbers will result in a negative number"));
    }

    // we will be mutating the values of op1 through borrowing, so
    // it must be cloned, op2 however can simply be a reference
    let mut op1 = lhs.clone();
    let op2 = &rhs;

    let mut result: Vec<u32> = Vec::new();

    // create a zipped ranged of indices with op1_range being the
    // parent, op2 is mapped with an option because if op2_range
    // is not as large as op1_range, it will simply fill it with
    // a None value
    let op1_range = (0..lhs.len()).rev();
    let op2_range = (0..rhs.len()).map(|x| Some(x)).rev();
    let zipped = op1_range.zip(op2_range.chain(repeat(None)));

    for (i, j) in zipped {
        // if there are no digits left in the second vector, then
        // simply make it zero
        let sub = 
            match j {
                Some(n) => { op2[n] },
                None    => { 0 }
            };

        // result of digit subtraction, Some means the result
        // can easily be obtained, None denotes a borrow must
        // take place
        let local_result: u32 = 
            match op1[i].checked_sub(sub) {
                Some(r) => { r },
                None    => {
                    // if the next digit can be borrowed(not zero)
                    // then borrow and return the correct result, otherwise
                    // loop until something can be borrowed.
                    let mut borrow = 1;
                    if op1[i-borrow] != 0 {
                        op1[i-borrow] -= 1;
                        op1[i] + 10 - sub
                    } else {
                        while op1[i-borrow] == 0 && borrow < op1.len() {
                            op1[i-borrow] = 9;
                            borrow += 1;
                        }
                        if op1[i-borrow] == 0 {
                            return Err(Error::new(ErrorType::UnsignedOverflow,
                                                  "Subtraction would result in negative result"));
                        }
                        op1[i-borrow] -= 1;
                        op1[i] + 10 - sub
                    }
                }
            };
        result.push(local_result);
    }

    Ok(result.into_iter().rev().collect::<Vec<_>>())
}
