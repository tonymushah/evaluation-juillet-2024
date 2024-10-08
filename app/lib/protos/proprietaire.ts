// @generated by protobuf-ts 2.9.4 with parameter server_none
// @generated from protobuf file "proprietaire.proto" (package "prorietaire", syntax proto3)
// tslint:disable
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message prorietaire.Empty
 */
export interface Empty {
}
/**
 * @generated from protobuf message prorietaire.LoginRequest
 */
export interface LoginRequest {
    /**
     * @generated from protobuf field: string telephone = 1;
     */
    telephone: string;
}
/**
 * @generated from protobuf message prorietaire.LoginResponse
 */
export interface LoginResponse {
    /**
     * @generated from protobuf field: string token = 1;
     */
    token: string;
}
/**
 * @generated from protobuf message prorietaire.ListBienRequest
 */
export interface ListBienRequest {
    /**
     * @generated from protobuf field: optional uint32 offset = 1;
     */
    offset?: number;
    /**
     * @generated from protobuf field: optional uint32 limit = 2;
     */
    limit?: number;
    /**
     * @generated from protobuf field: repeated string type_biens = 3;
     */
    typeBiens: string[];
}
/**
 * @generated from protobuf message prorietaire.Bien
 */
export interface Bien {
    /**
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * @generated from protobuf field: string nom = 2;
     */
    nom: string;
    /**
     * @generated from protobuf field: string region = 3;
     */
    region: string;
    /**
     * @generated from protobuf field: string description = 4;
     */
    description: string;
    /**
     * string proprietaire = 5;
     *
     * @generated from protobuf field: prorietaire.Bien.TypeBien type_bien = 6;
     */
    typeBien?: Bien_TypeBien;
    /**
     * bool is_disponible = 7;
     *
     * @generated from protobuf field: string date_disponible = 8;
     */
    dateDisponible: string;
}
/**
 * @generated from protobuf message prorietaire.Bien.TypeBien
 */
export interface Bien_TypeBien {
    /**
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * @generated from protobuf field: string nom = 2;
     */
    nom: string;
}
/**
 * @generated from protobuf message prorietaire.ListBienResponse
 */
export interface ListBienResponse {
    /**
     * @generated from protobuf field: repeated prorietaire.Bien biens = 1;
     */
    biens: Bien[];
    /**
     * @generated from protobuf field: uint64 offset = 2;
     */
    offset: bigint;
    /**
     * @generated from protobuf field: uint64 limit = 3;
     */
    limit: bigint;
    /**
     * @generated from protobuf field: uint64 total = 4;
     */
    total: bigint;
}
/**
 * @generated from protobuf message prorietaire.GetBienRequest
 */
export interface GetBienRequest {
    /**
     * @generated from protobuf field: string id = 1;
     */
    id: string;
}
/**
 * @generated from protobuf message prorietaire.ListTypeBienRequest
 */
export interface ListTypeBienRequest {
    /**
     * @generated from protobuf field: optional uint32 offset = 1;
     */
    offset?: number;
    /**
     * @generated from protobuf field: optional uint32 limit = 2;
     */
    limit?: number;
}
/**
 * @generated from protobuf message prorietaire.ListTypeBienResponse
 */
export interface ListTypeBienResponse {
    /**
     * @generated from protobuf field: repeated prorietaire.Bien.TypeBien data = 1;
     */
    data: Bien_TypeBien[];
    /**
     * @generated from protobuf field: uint64 offset = 2;
     */
    offset: bigint;
    /**
     * @generated from protobuf field: uint64 limit = 3;
     */
    limit: bigint;
    /**
     * @generated from protobuf field: uint64 total = 4;
     */
    total: bigint;
}
/**
 * @generated from protobuf message prorietaire.ChiffreAffairesRequest
 */
export interface ChiffreAffairesRequest {
    /**
     * @generated from protobuf field: repeated string biens = 1;
     */
    biens: string[];
    /**
     * @generated from protobuf field: repeated string type_bien = 2;
     */
    typeBien: string[];
}
/**
 * @generated from protobuf message prorietaire.ChiffreAffairesResponse
 */
export interface ChiffreAffairesResponse {
    /**
     * @generated from protobuf field: float valeur = 1;
     */
    valeur: number;
}
// @generated message type with reflection information, may provide speed optimized methods
class Empty$Type extends MessageType<Empty> {
    constructor() {
        super("prorietaire.Empty", []);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.Empty
 */
export const Empty = new Empty$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginRequest$Type extends MessageType<LoginRequest> {
    constructor() {
        super("prorietaire.LoginRequest", [
            { no: 1, name: "telephone", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.LoginRequest
 */
export const LoginRequest = new LoginRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginResponse$Type extends MessageType<LoginResponse> {
    constructor() {
        super("prorietaire.LoginResponse", [
            { no: 1, name: "token", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.LoginResponse
 */
export const LoginResponse = new LoginResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListBienRequest$Type extends MessageType<ListBienRequest> {
    constructor() {
        super("prorietaire.ListBienRequest", [
            { no: 1, name: "offset", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 2, name: "limit", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "type_biens", kind: "scalar", repeat: 2 /*RepeatType.UNPACKED*/, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.ListBienRequest
 */
export const ListBienRequest = new ListBienRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class Bien$Type extends MessageType<Bien> {
    constructor() {
        super("prorietaire.Bien", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "nom", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "region", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "description", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 6, name: "type_bien", kind: "message", T: () => Bien_TypeBien },
            { no: 8, name: "date_disponible", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.Bien
 */
export const Bien = new Bien$Type();
// @generated message type with reflection information, may provide speed optimized methods
class Bien_TypeBien$Type extends MessageType<Bien_TypeBien> {
    constructor() {
        super("prorietaire.Bien.TypeBien", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "nom", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.Bien.TypeBien
 */
export const Bien_TypeBien = new Bien_TypeBien$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListBienResponse$Type extends MessageType<ListBienResponse> {
    constructor() {
        super("prorietaire.ListBienResponse", [
            { no: 1, name: "biens", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Bien },
            { no: 2, name: "offset", kind: "scalar", T: 4 /*ScalarType.UINT64*/, L: 0 /*LongType.BIGINT*/ },
            { no: 3, name: "limit", kind: "scalar", T: 4 /*ScalarType.UINT64*/, L: 0 /*LongType.BIGINT*/ },
            { no: 4, name: "total", kind: "scalar", T: 4 /*ScalarType.UINT64*/, L: 0 /*LongType.BIGINT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.ListBienResponse
 */
export const ListBienResponse = new ListBienResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetBienRequest$Type extends MessageType<GetBienRequest> {
    constructor() {
        super("prorietaire.GetBienRequest", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.GetBienRequest
 */
export const GetBienRequest = new GetBienRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListTypeBienRequest$Type extends MessageType<ListTypeBienRequest> {
    constructor() {
        super("prorietaire.ListTypeBienRequest", [
            { no: 1, name: "offset", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 2, name: "limit", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.ListTypeBienRequest
 */
export const ListTypeBienRequest = new ListTypeBienRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListTypeBienResponse$Type extends MessageType<ListTypeBienResponse> {
    constructor() {
        super("prorietaire.ListTypeBienResponse", [
            { no: 1, name: "data", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => Bien_TypeBien },
            { no: 2, name: "offset", kind: "scalar", T: 4 /*ScalarType.UINT64*/, L: 0 /*LongType.BIGINT*/ },
            { no: 3, name: "limit", kind: "scalar", T: 4 /*ScalarType.UINT64*/, L: 0 /*LongType.BIGINT*/ },
            { no: 4, name: "total", kind: "scalar", T: 4 /*ScalarType.UINT64*/, L: 0 /*LongType.BIGINT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.ListTypeBienResponse
 */
export const ListTypeBienResponse = new ListTypeBienResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ChiffreAffairesRequest$Type extends MessageType<ChiffreAffairesRequest> {
    constructor() {
        super("prorietaire.ChiffreAffairesRequest", [
            { no: 1, name: "biens", kind: "scalar", repeat: 2 /*RepeatType.UNPACKED*/, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "type_bien", kind: "scalar", repeat: 2 /*RepeatType.UNPACKED*/, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.ChiffreAffairesRequest
 */
export const ChiffreAffairesRequest = new ChiffreAffairesRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ChiffreAffairesResponse$Type extends MessageType<ChiffreAffairesResponse> {
    constructor() {
        super("prorietaire.ChiffreAffairesResponse", [
            { no: 1, name: "valeur", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message prorietaire.ChiffreAffairesResponse
 */
export const ChiffreAffairesResponse = new ChiffreAffairesResponse$Type();
/**
 * @generated ServiceType for protobuf service prorietaire.Auth
 */
export const Auth = new ServiceType("prorietaire.Auth", [
    { name: "Login", options: {}, I: LoginRequest, O: LoginResponse }
]);
/**
 * @generated ServiceType for protobuf service prorietaire.Biens
 */
export const Biens = new ServiceType("prorietaire.Biens", [
    { name: "ListBien", options: {}, I: ListBienRequest, O: ListBienResponse },
    { name: "Get", options: {}, I: GetBienRequest, O: Bien },
    { name: "ListTypeBien", options: {}, I: ListTypeBienRequest, O: ListTypeBienResponse }
]);
/**
 * @generated ServiceType for protobuf service prorietaire.Comptes
 */
export const Comptes = new ServiceType("prorietaire.Comptes", [
    { name: "ChiffreAffaires", options: {}, I: ChiffreAffairesRequest, O: ChiffreAffairesResponse }
]);
