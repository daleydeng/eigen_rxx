from easydict import EasyDict as edict

from_ctype = {
    "double": "f64",
    "float": "f32",
    "int": "i32",
}

postfix = {
    'double': 'd',
    'float': 'f',
    'int': 'i'
}

def link_name(name, cls=''):
    if not cls:
        return f"eigen_rxx${name}"
    return f"eigen_rxx${cls}${name}"

def build_Map_new(cls, tp):
    return {
        link_name(f"MapMut_{cls}_new"): {
            "c_sig": {
                "fn": f"MapMut_fixed_new<{cls}, {tp}>",
                "ret_type": f"Eigen::Map<{cls}>",
                "args": [
                    (f"{tp} *", "data"),
                ],
            },
            "rs_sig": {
                "fn": f"MapMut_{cls}_new<'a>",
                "ret_type": f"MapMut_{cls}<'a>",
                "fn_tpl": "<'a>",
            },
        },
        link_name(f"Map_{cls}_new"): {
            "c_sig": {
                "fn": f"Map_fixed_new<{cls}, {tp}>",
                "ret_type": f"Eigen::Map<{cls} const>",
                "args": [
                    (f"{tp} const*", "data"),
                ],
            },
            "rs_sig": {
                "fn": f"Map_{cls}_new<'a>",
                "ret_type": f"Map_{cls}<'a>",
                "fn_tpl": "<'a>",
            },
        },
    }

def get_globals():
    fns = {}
    mem_fns = {}
    fixed_matrices = {}
    for tp in ["double", "float", "int"]:
        t = postfix[tp]
        rs_tp = from_ctype[tp]
        for R, C in [(2, 2), (3, 3), (4, 4), (2, 3), (2, 1), (3, 1), (4, 1)]:
            kws = {}
            if C == 1:
                cls = f'Vector{R}{t}'
                rs_own = f'na::Vector{R}'
                rs_ref = f'na::VectorSlice{R}'
                rs_mut_ref = f'na::VectorSliceMut{R}'

            elif R == C:
                cls = f'Matrix{R}{t}'
                rs_own = f'na::Matrix{R}'
                rs_ref = f'na::MatrixSlice{R}'
                rs_mut_ref = f'na::MatrixSliceMut{R}'

            else:
                cls = f'Matrix{R}x{C}{t}'
                rs_own = f'na::Matrix{R}x{C}'
                rs_ref = f'na::MatrixSlice{R}x{C}'
                rs_mut_ref = f'na::MatrixSliceMut{R}x{C}'

                kws.update({
                    'mul': True,
                })

            size = R * C
            fixed_matrices.update({
                cls: {
                    'shape': (R, C),
                    "size": size,
                    'tp': rs_tp,
                    'rs_own': rs_own,
                    'rs_ref': rs_ref,
                    'rs_mut_ref': rs_mut_ref,
                    **kws
                }
            })

            fns.update(build_Map_new(cls, tp))

    quats = {}
    aas = {}
    for tp in ["double", "float"]:
        t = postfix[tp]
        cls = f'Quaternion{t}'

        rs_tp = from_ctype[tp]

        quats.update({
            cls: {
                "tp": rs_tp,
                "rs_own": "na::Quaternion",
            }
        })

        mem_fns.update({
            link_name("normalized", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "&$C::normalized",
                    "ret_type": "$C",
                },
                "rs_sig": {}
            },
            link_name("normalize", cls): {
                "cls": cls,
                "is_const": False,
                "c_sig": {
                    "fn": "&$C::normalize",
                },
                "rs_sig": {}
            },
            link_name("inverse", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "&$C::inverse",
                    "ret_type": "$C",
                },
                "rs_sig": {}
            },
            link_name("mul", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "op_mul",
                    "ret_type": "$C",
                    "args": [
                        ("$C const&", "other"),
                    ],
                },
                "rs_sig": {
                    "fn": "mul",
                },
            },
            link_name("toRotationMatrix", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "&$C::toRotationMatrix",
                    "ret_type": f"Matrix3{t}",
                }
            },
        })

        fns.update(build_Map_new(cls, tp))

        quat_cls = cls
        cls = f'AngleAxis{t}'
        aas.update({
            cls: {
                'tp': rs_tp,
                'axis': f"Vector3{t}",
            }
        })

        mem_fns.update({
            link_name("inverse", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "&$C::inverse",
                    "ret_type": "$C",
                },
                "rs_sig": {}
            },
            link_name("mul", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "op_mul",
                    "ret_type": quat_cls,
                    "args": [
                        ("$C const&", "other"),
                    ],
                },
                "rs_sig": {
                    "fn": "mul",
                },
            },
            link_name("toRotationMatrix", cls): {
                "cls": cls,
                "c_sig": {
                    "fn": "&$C::toRotationMatrix",
                    "ret_type": f"Matrix3{t}",
                }
            },
        })

        # AngleAxis doesn't support Map

    return edict({
        "type_mapping": from_ctype,
        "cffi_fixed_matrices": fixed_matrices,
        "cffi_quats": quats,
        "cffi_aas": aas,
        "cffi_mem_fns": mem_fns,
        "cffi_fns": fns,
    })
