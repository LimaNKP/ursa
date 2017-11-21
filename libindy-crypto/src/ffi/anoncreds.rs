use anoncreds::*;
use anoncreds::issuer::*;
use anoncreds::types::*;

use ffi::ErrorCode;
use ffi::indy_crypto_init_logger;
use errors::ToErrorCode;
use utils::ctypes::CTypesUtils;

use libc::c_char;

use std::os::raw::c_void;
use std::slice;


/// Creates and returns claims attributes entity builder.
///
/// The purpose of claim attributes builder is building of claim attributes entity that
/// represents claim attributes set.
///
/// Note: Claims attributes builder instance deallocation must be performed by
/// calling indy_crypto_anoncreds_claim_attributes_builder_finalize.
///
/// # Arguments
/// * `claim_attrs_builder_p` - Reference that will contain claims attributes builder instance pointer.
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_builder_new(claim_attrs_builder_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_builder_new: >>> claim_attrs_builder_p: {:?}", claim_attrs_builder_p);

    check_useful_c_ptr!(claim_attrs_builder_p, ErrorCode::CommonInvalidParam1);

    let res = match ClaimAttributesBuilder::new() {
        Ok(claim_attrs_builder) => {
            trace!("indy_crypto_anoncreds_claim_attributes_builder_new: claim_attrs_builder: {:?}", claim_attrs_builder);
            unsafe {
                *claim_attrs_builder_p = Box::into_raw(Box::new(claim_attrs_builder)) as *const c_void;
                trace!("indy_crypto_anoncreds_claim_attributes_builder_new: *claim_attrs_builder_p: {:?}", *claim_attrs_builder_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_anoncreds_claim_attributes_builder_new: <<< res: {:?}", res);
    res
}

/// Adds new attribute to claim attributes set.
///
/// Note that this function returns new claim attribute builder instance pointer. The old one
/// becomes invalid.
///
/// # Arguments
/// * `claim_attrs_builder` - Claim attribute builder instance pointer
/// * `attr` - Claim attr to add as null terminated string.
/// * `claim_attrs_builder_p` - Reference that will contain new claims attributes builder instance pointer.
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder: *const c_void,
                                                                      attr: *const c_char,
                                                                      claim_attrs_builder_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_builder_add_attr: >>> claim_attrs_builder: {:?}, attr: {:?}, claim_attrs_builder_p: {:?}", claim_attrs_builder, attr, claim_attrs_builder_p);

    check_useful_c_ptr!(claim_attrs_builder, ErrorCode::CommonInvalidParam1);
    check_useful_c_str!(attr, ErrorCode::CommonInvalidParam2);
    check_useful_c_ptr!(claim_attrs_builder_p, ErrorCode::CommonInvalidParam1);

    let mut claim_attrs_builder = unsafe { Box::from_raw(claim_attrs_builder as *mut ClaimAttributesBuilder) };

    let res = match claim_attrs_builder.add_attr(&attr) {
        Ok(claim_attrs_builder) => {
            trace!("indy_crypto_anoncreds_claim_attributes_builder_add_attr: claim_attrs_builder: {:?}", claim_attrs_builder);
            unsafe {
                *claim_attrs_builder_p = Box::into_raw(Box::new(claim_attrs_builder)) as *const c_void;
                trace!("indy_crypto_anoncreds_claim_attributes_builder_add_attr: *claim_attrs_builder_p: {:?}", *claim_attrs_builder_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_anoncreds_claim_attributes_builder_add_attr: <<< res: {:?}", res);
    res
}

/// Deallocates claim attribute builder and returns claim attributes entity instead.
///
/// Note: Claims attributes builder instance deallocation must be performed by
/// calling indy_crypto_anoncreds_claim_attributes_free.
///
/// # Arguments
/// * `claim_attrs_builder` - Claim attribute builder instance pointer
/// * `claim_attrs_p` - Reference that will contain claims attributes instance pointer.
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_builder_finalize(claim_attrs_builder: *const c_void,
                                                                      claim_attrs_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_builder_finalize: >>> claim_attrs_builder: {:?}, claim_attrs_p: {:?}", claim_attrs_builder, claim_attrs_p);

    check_useful_c_ptr!(claim_attrs_builder, ErrorCode::CommonInvalidParam1);
    check_useful_c_ptr!(claim_attrs_p, ErrorCode::CommonInvalidParam2);

    let claim_attrs_builder = unsafe { Box::from_raw(claim_attrs_builder as *mut ClaimAttributesBuilder) };

    let res = match claim_attrs_builder.finalize() {
        Ok(claims_attrs) => {
            trace!("indy_crypto_anoncreds_claim_attributes_builder_finalize: claims_attrs: {:?}", claims_attrs);
            unsafe {
                *claim_attrs_p = Box::into_raw(Box::new(claims_attrs)) as *const c_void;
                trace!("indy_crypto_anoncreds_claim_attributes_builder_finalize: *claim_attrs_p: {:?}", *claim_attrs_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_anoncreds_claim_attributes_builder_finalize: <<< res: {:?}", res);
    res
}

/// Deallocates claim attributes instance.
///
/// # Arguments
/// * `claims_attrs` - Claim attributes instance pointer
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_free(claims_attrs: *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_free: >>> claims_attrs: {:?}", claims_attrs);

    check_useful_c_ptr!(claims_attrs, ErrorCode::CommonInvalidParam1);

    unsafe { Box::from_raw(claims_attrs as *mut ClaimAttributes); }
    let res = ErrorCode::Success;

    trace!("indy_crypto_anoncreds_claim_attributes_free: <<< res: {:?}", res);
    res
}

/// Creates and returns claims attributes values entity builder.
///
/// The purpose of claim attributes values builder is building of claim attributes values entity that
/// represents claim attributes values map.
///
/// Note: Claims attributes values builder instance deallocation must be performed by
/// calling indy_crypto_anoncreds_claim_attributes_values_builder_finalize.
///
/// # Arguments
/// * `claim_attrs_values_builder_p` - Reference that will contain claims attributes values builder instance pointer.
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_values_builder_new(claim_attrs_values_builder_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_values_builder_new: >>> claim_attrs_values_builder_p: {:?}", claim_attrs_values_builder_p);

    check_useful_c_ptr!(claim_attrs_values_builder_p, ErrorCode::CommonInvalidParam1);

    let res = match ClaimAttributesValuesBuilder::new() {
        Ok(claim_attrs_values_builder) => {
            trace!("indy_crypto_anoncreds_claim_attributes_values_builder_new: claim_attrs_values_builder: {:?}", claim_attrs_values_builder);
            unsafe {
                *claim_attrs_values_builder_p = Box::into_raw(Box::new(claim_attrs_values_builder)) as *const c_void;
                trace!("indy_crypto_anoncreds_claim_attributes_values_builder_new: *claim_attrs_values_builder_p: {:?}", *claim_attrs_values_builder_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_anoncreds_claim_attributes_values_builder_new: <<< res: {:?}", res);
    res
}

/// Adds new attribute dec_value to claim attributes values map.
///
/// Note that this function returns new claim attribute dec_value builder instance pointer. The old one
/// becomes invalid.
///
/// # Arguments
/// * `claim_attrs_values_builder` - Claim attributes values builder instance pointer
/// * `attr` - Claim attr to add as null terminated string.
/// * `dec_value` - Claim attr dec_value. Decimal BigNum representation as null terminated string.
/// * `claim_attrs_values_builder_p` - Reference that will contain new claims attributes builder instance pointer.
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value(claim_attrs_values_builder: *const c_void,
                                                                                   attr: *const c_char,
                                                                                   dec_value: *const c_char,
                                                                                   claim_attrs_values_builder_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value: >>> claim_attrs_values_builder: {:?}, attr: {:?}, dec_value: {:?}, claim_attrs_values_builder_p: {:?}", claim_attrs_values_builder, attr, dec_value, claim_attrs_values_builder_p);

    check_useful_c_ptr!(claim_attrs_values_builder, ErrorCode::CommonInvalidParam1);
    check_useful_c_str!(attr, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(dec_value, ErrorCode::CommonInvalidParam3);
    check_useful_c_ptr!(claim_attrs_values_builder_p, ErrorCode::CommonInvalidParam4);

    let mut claim_attrs_values_builder = unsafe { Box::from_raw(claim_attrs_values_builder as *mut ClaimAttributesValuesBuilder) };

    let res = match claim_attrs_values_builder.add_attr_value(&attr, &dec_value) {
        Ok(claim_attrs_values_builder) => {
            trace!("indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value: claim_attrs_values_builder: {:?}", claim_attrs_values_builder);
            unsafe {
                *claim_attrs_values_builder_p = Box::into_raw(Box::new(claim_attrs_values_builder)) as *const c_void;
                trace!("indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value: *claim_attrs_values_builder_p: {:?}", *claim_attrs_values_builder_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value: <<< res: {:?}", res);
    res
}

/// Deallocates claim attribute values builder and returns claim attributes values entity instead.
///
/// Note: Claims attributes values builder instance deallocation must be performed by
/// calling indy_crypto_anoncreds_claim_attributes_values_free.
///
/// # Arguments
/// * `claim_attrs_values_builder` - Claim attribute builder instance pointer
/// * `claim_attrs_values_p` - Reference that will contain claims attributes values instance pointer.
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_values_builder_finalize(claim_attrs_values_builder: *const c_void,
                                                                             claim_attrs_values_p: *mut *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_values_builder_finalize: >>> claim_attrs_values_builder: {:?}, claim_attrs_values_p: {:?}", claim_attrs_values_builder, claim_attrs_values_p);

    check_useful_c_ptr!(claim_attrs_values_builder, ErrorCode::CommonInvalidParam1);
    check_useful_c_ptr!(claim_attrs_values_p, ErrorCode::CommonInvalidParam2);

    let claim_attrs_values_builder = unsafe { Box::from_raw(claim_attrs_values_builder as *mut ClaimAttributesValuesBuilder) };

    let res = match claim_attrs_values_builder.finalize() {
        Ok(claims_attrs_values) => {
            trace!("indy_crypto_anoncreds_claim_attributes_values_builder_finalize: claims_attrs_values: {:?}", claims_attrs_values);
            unsafe {
                *claim_attrs_values_p = Box::into_raw(Box::new(claims_attrs_values)) as *const c_void;
                trace!("indy_crypto_anoncreds_claim_attributes_values_builder_finalize: *claim_attrs_values_p: {:?}", *claim_attrs_values_p);
            }
            ErrorCode::Success
        }
        Err(err) => err.to_error_code()
    };

    trace!("indy_crypto_anoncreds_claim_attributes_values_builder_finalize: <<< res: {:?}", res);
    res
}

/// Deallocates claim attributes мфдгуы instance.
///
/// # Arguments
/// * `claims_attrs_values` - Claim attributes values instance pointer
#[no_mangle]
pub extern fn indy_crypto_anoncreds_claim_attributes_values_free(claims_attrs_values: *const c_void) -> ErrorCode {
    trace!("indy_crypto_anoncreds_claim_attributes_values_free: >>> claims_attrs_values: {:?}", claims_attrs_values);

    check_useful_c_ptr!(claims_attrs_values, ErrorCode::CommonInvalidParam1);

    unsafe { Box::from_raw(claims_attrs_values as *mut ClaimAttributesValues); }
    let res = ErrorCode::Success;

    trace!("indy_crypto_anoncreds_claim_attributes_values_free: <<< res: {:?}", res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn indy_crypto_anoncreds_claim_attributes_builder_new_works() {
        let mut claim_attrs_builder: *const c_void = ptr::null();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_new(&mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let mut claim_attrs: *const c_void = ptr::null();
        indy_crypto_anoncreds_claim_attributes_builder_finalize(claim_attrs_builder, &mut claim_attrs);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs.is_null());

        indy_crypto_anoncreds_claim_attributes_free(claim_attrs);
        assert_eq!(err_code, ErrorCode::Success);
    }

    #[test]
    fn indy_crypto_anoncreds_claim_attributes_builder_add_attr_works() {
        let mut claim_attrs_builder: *const c_void = ptr::null();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_new(&mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let attr = CString::new("sex").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder, attr.as_ptr(), &mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let attr = CString::new("name").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder, attr.as_ptr(), &mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let attr = CString::new("age").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder, attr.as_ptr(), &mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let mut claim_attrs: *const c_void = ptr::null();
        indy_crypto_anoncreds_claim_attributes_builder_finalize(claim_attrs_builder, &mut claim_attrs);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs.is_null());

        indy_crypto_anoncreds_claim_attributes_free(claim_attrs);
        assert_eq!(err_code, ErrorCode::Success);
    }

    #[test]
    fn indy_crypto_anoncreds_claim_attributes_builder_finalize_works() {
        let mut claim_attrs_builder: *const c_void = ptr::null();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_new(&mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let attr = CString::new("sex").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder, attr.as_ptr(), &mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let attr = CString::new("name").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder, attr.as_ptr(), &mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let attr = CString::new("age").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_add_attr(claim_attrs_builder, attr.as_ptr(), &mut claim_attrs_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_builder.is_null());

        let mut claim_attrs: *const c_void = ptr::null();
        indy_crypto_anoncreds_claim_attributes_builder_finalize(claim_attrs_builder, &mut claim_attrs);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs.is_null());

        indy_crypto_anoncreds_claim_attributes_free(claim_attrs);
        assert_eq!(err_code, ErrorCode::Success);
    }

    #[test]
    fn indy_crypto_anoncreds_claim_attributes_values_builder_new_works() {
        let mut claim_attrs_values_builder: *const c_void = ptr::null();
        let err_code = indy_crypto_anoncreds_claim_attributes_values_builder_new(&mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());

        let mut claim_attrs_values: *const c_void = ptr::null();
        indy_crypto_anoncreds_claim_attributes_builder_finalize(claim_attrs_values_builder, &mut claim_attrs_values);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values.is_null());

        indy_crypto_anoncreds_claim_attributes_free(claim_attrs_values);
        assert_eq!(err_code, ErrorCode::Success);
    }

    #[test]
    fn indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value_works() {
        let mut claim_attrs_values_builder: *const c_void = ptr::null();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_new(&mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());

        let attr = CString::new("sex").unwrap();
        let dec_value = CString::new("89057765651800459030103911598694169835931320404459570102253965466045532669865684092518362135930940112502263498496335250135601124519172068317163741086983519494043168252186111551835366571584950296764626458785776311514968350600732183408950813066589742888246925358509482561838243805468775416479523402043160919428168650069477488093758569936116799246881809224343325540306266957664475026390533069487455816053169001876208052109360113102565642529699056163373190930839656498261278601357214695582219007449398650197048218304260447909283768896882743373383452996855450316360259637079070460616248922547314789644935074980711243164129").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value(claim_attrs_values_builder,
                                                                                            attr.as_ptr(),
                                                                                            dec_value.as_ptr(),
                                                                                            &mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());

        let attr = CString::new("name").unwrap();
        let dec_value = CString::new("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value(claim_attrs_values_builder,
                                                                                            attr.as_ptr(),
                                                                                            dec_value.as_ptr(),
                                                                                            &mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());



        let mut claim_attrs_values: *const c_void = ptr::null();
        indy_crypto_anoncreds_claim_attributes_values_builder_finalize(claim_attrs_values_builder, &mut claim_attrs_values);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values.is_null());

        indy_crypto_anoncreds_claim_attributes_values_free(claim_attrs_values);
        assert_eq!(err_code, ErrorCode::Success);
    }

    #[test]
    fn indy_crypto_anoncreds_claim_attributes_values_builder_free_works() {
        let mut claim_attrs_values_builder: *const c_void = ptr::null();
        let err_code = indy_crypto_anoncreds_claim_attributes_builder_new(&mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());

        let attr = CString::new("sex").unwrap();
        let dec_value = CString::new("89057765651800459030103911598694169835931320404459570102253965466045532669865684092518362135930940112502263498496335250135601124519172068317163741086983519494043168252186111551835366571584950296764626458785776311514968350600732183408950813066589742888246925358509482561838243805468775416479523402043160919428168650069477488093758569936116799246881809224343325540306266957664475026390533069487455816053169001876208052109360113102565642529699056163373190930839656498261278601357214695582219007449398650197048218304260447909283768896882743373383452996855450316360259637079070460616248922547314789644935074980711243164129").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value(claim_attrs_values_builder,
                                                                                            attr.as_ptr(),
                                                                                            dec_value.as_ptr(),
                                                                                            &mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());

        let attr = CString::new("name").unwrap();
        let dec_value = CString::new("58606710922154038918005745652863947546479611221487923871520854046018234465128105585608812090213473225037875788462225679336791123783441657062831589984290779844020407065450830035885267846722229953206567087435754612694085258455822926492275621650532276267042885213400704012011608869094703483233081911010530256094461587809601298503874283124334225428746479707531278882536314925285434699376158578239556590141035593717362562548075653598376080466948478266094753818404986494459240364648986755479857098110402626477624280802323635285059064580583239726433768663879431610261724430965980430886959304486699145098822052003020688956471").unwrap();
        let err_code = indy_crypto_anoncreds_claim_attributes_values_builder_add_attr_value(claim_attrs_values_builder,
                                                                                            attr.as_ptr(),
                                                                                            dec_value.as_ptr(),
                                                                                            &mut claim_attrs_values_builder);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values_builder.is_null());



        let mut claim_attrs_values: *const c_void = ptr::null();
        indy_crypto_anoncreds_claim_attributes_values_builder_finalize(claim_attrs_values_builder, &mut claim_attrs_values);
        assert_eq!(err_code, ErrorCode::Success);
        assert!(!claim_attrs_values.is_null());

        indy_crypto_anoncreds_claim_attributes_values_free(claim_attrs_values);
        assert_eq!(err_code, ErrorCode::Success);
    }
}