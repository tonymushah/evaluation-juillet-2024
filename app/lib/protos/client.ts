// @generated by protobuf-ts 2.9.4 with parameter server_none
// @generated from protobuf file "client.proto" (package "client", syntax proto3)
// tslint:disable
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message client.Empty
 */
export interface Empty {
}
/**
 * @generated from protobuf message client.LoginRequest
 */
export interface LoginRequest {
    /**
     * @generated from protobuf field: string email = 1;
     */
    email: string;
}
/**
 * @generated from protobuf message client.LoginResponse
 */
export interface LoginResponse {
    /**
     * @generated from protobuf field: string token = 1;
     */
    token: string;
}
/**
 * @generated from protobuf message client.LoyersRequest
 */
export interface LoyersRequest {
    /**
     * @generated from protobuf field: optional string date_debut = 1;
     */
    dateDebut?: string;
    /**
     * @generated from protobuf field: optional string date_fin = 2;
     */
    dateFin?: string;
    /**
     * @generated from protobuf field: optional string bien = 3;
     */
    bien?: string;
    /**
     * @generated from protobuf field: optional string location = 4;
     */
    location?: string;
}
/**
 * @generated from protobuf message client.LoyersResponse
 */
export interface LoyersResponse {
    /**
     * @generated from protobuf field: float total = 1;
     */
    total: number;
}
/**
 * @generated from protobuf message client.LoyersPayeRequest
 */
export interface LoyersPayeRequest {
    /**
     * @generated from protobuf field: optional string date_debut = 1;
     */
    dateDebut?: string;
    /**
     * @generated from protobuf field: optional string date_fin = 2;
     */
    dateFin?: string;
    /**
     * @generated from protobuf field: optional string bien = 3;
     */
    bien?: string;
    /**
     * @generated from protobuf field: optional string location = 4;
     */
    location?: string;
}
/**
 * @generated from protobuf message client.LoyersPayeResponse
 */
export interface LoyersPayeResponse {
    /**
     * @generated from protobuf field: float total = 1;
     */
    total: number;
}
/**
 * @generated from protobuf message client.FairePayementRequest
 */
export interface FairePayementRequest {
    /**
     * @generated from protobuf field: float motant = 2;
     */
    motant: number;
    /**
     * @generated from protobuf field: string location = 3;
     */
    location: string;
}
/**
 * @generated from protobuf message client.ListLocationRequest
 */
export interface ListLocationRequest {
    /**
     * @generated from protobuf field: optional string bien = 1;
     */
    bien?: string;
    /**
     * @generated from protobuf field: optional uint32 offset = 2;
     */
    offset?: number;
    /**
     * @generated from protobuf field: optional uint32 limit = 3;
     */
    limit?: number;
}
/**
 * @generated from protobuf message client.ListLocationResponse
 */
export interface ListLocationResponse {
    /**
     * @generated from protobuf field: repeated client.ListLocationResponse.Location locations = 1;
     */
    locations: ListLocationResponse_Location[];
    /**
     * @generated from protobuf field: uint32 offset = 2;
     */
    offset: number;
    /**
     * @generated from protobuf field: uint32 limit = 3;
     */
    limit: number;
    /**
     * @generated from protobuf field: uint32 total = 4;
     */
    total: number;
}
/**
 * @generated from protobuf message client.ListLocationResponse.Location
 */
export interface ListLocationResponse_Location {
    /**
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * @generated from protobuf field: string date_debut = 2;
     */
    dateDebut: string;
    /**
     * @generated from protobuf field: string date_fin = 3;
     */
    dateFin: string;
    /**
     * @generated from protobuf field: string bien = 4;
     */
    bien: string;
}
// @generated message type with reflection information, may provide speed optimized methods
class Empty$Type extends MessageType<Empty> {
    constructor() {
        super("client.Empty", []);
    }
}
/**
 * @generated MessageType for protobuf message client.Empty
 */
export const Empty = new Empty$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginRequest$Type extends MessageType<LoginRequest> {
    constructor() {
        super("client.LoginRequest", [
            { no: 1, name: "email", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.LoginRequest
 */
export const LoginRequest = new LoginRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginResponse$Type extends MessageType<LoginResponse> {
    constructor() {
        super("client.LoginResponse", [
            { no: 1, name: "token", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.LoginResponse
 */
export const LoginResponse = new LoginResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoyersRequest$Type extends MessageType<LoyersRequest> {
    constructor() {
        super("client.LoyersRequest", [
            { no: 1, name: "date_debut", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "date_fin", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "bien", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "location", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.LoyersRequest
 */
export const LoyersRequest = new LoyersRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoyersResponse$Type extends MessageType<LoyersResponse> {
    constructor() {
        super("client.LoyersResponse", [
            { no: 1, name: "total", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.LoyersResponse
 */
export const LoyersResponse = new LoyersResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoyersPayeRequest$Type extends MessageType<LoyersPayeRequest> {
    constructor() {
        super("client.LoyersPayeRequest", [
            { no: 1, name: "date_debut", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "date_fin", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "bien", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "location", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.LoyersPayeRequest
 */
export const LoyersPayeRequest = new LoyersPayeRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoyersPayeResponse$Type extends MessageType<LoyersPayeResponse> {
    constructor() {
        super("client.LoyersPayeResponse", [
            { no: 1, name: "total", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.LoyersPayeResponse
 */
export const LoyersPayeResponse = new LoyersPayeResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class FairePayementRequest$Type extends MessageType<FairePayementRequest> {
    constructor() {
        super("client.FairePayementRequest", [
            { no: 2, name: "motant", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ },
            { no: 3, name: "location", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.FairePayementRequest
 */
export const FairePayementRequest = new FairePayementRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListLocationRequest$Type extends MessageType<ListLocationRequest> {
    constructor() {
        super("client.ListLocationRequest", [
            { no: 1, name: "bien", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "offset", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "limit", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.ListLocationRequest
 */
export const ListLocationRequest = new ListLocationRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListLocationResponse$Type extends MessageType<ListLocationResponse> {
    constructor() {
        super("client.ListLocationResponse", [
            { no: 1, name: "locations", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => ListLocationResponse_Location },
            { no: 2, name: "offset", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "limit", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
            { no: 4, name: "total", kind: "scalar", T: 13 /*ScalarType.UINT32*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.ListLocationResponse
 */
export const ListLocationResponse = new ListLocationResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListLocationResponse_Location$Type extends MessageType<ListLocationResponse_Location> {
    constructor() {
        super("client.ListLocationResponse.Location", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "date_debut", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "date_fin", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "bien", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message client.ListLocationResponse.Location
 */
export const ListLocationResponse_Location = new ListLocationResponse_Location$Type();
/**
 * @generated ServiceType for protobuf service client.Auth
 */
export const Auth = new ServiceType("client.Auth", [
    { name: "Login", options: {}, I: LoginRequest, O: LoginResponse }
]);
/**
 * @generated ServiceType for protobuf service client.Payement
 */
export const Payement = new ServiceType("client.Payement", [
    { name: "Loyers", options: {}, I: LoyersRequest, O: LoyersResponse },
    { name: "LoyersPaye", options: {}, I: LoyersPayeRequest, O: LoyersPayeResponse },
    { name: "FairePayement", options: {}, I: FairePayementRequest, O: Empty }
]);
/**
 * @generated ServiceType for protobuf service client.Location
 */
export const Location = new ServiceType("client.Location", [
    { name: "List", options: {}, I: ListLocationRequest, O: ListLocationResponse }
]);
