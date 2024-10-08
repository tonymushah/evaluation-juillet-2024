// @generated by protobuf-ts 2.9.4 with parameter server_none
// @generated from protobuf file "admin.proto" (package "admin", syntax proto3)
// tslint:disable
import { ServiceType } from "@protobuf-ts/runtime-rpc";
import { MessageType } from "@protobuf-ts/runtime";
/**
 * @generated from protobuf message admin.Empty
 */
export interface Empty {
}
/**
 * @generated from protobuf message admin.LoginRequest
 */
export interface LoginRequest {
    /**
     * @generated from protobuf field: string password = 2;
     */
    password: string;
}
/**
 * @generated from protobuf message admin.LoginResponse
 */
export interface LoginResponse {
    /**
     * @generated from protobuf field: string session_token = 1;
     */
    sessionToken: string;
}
/**
 * @generated from protobuf message admin.ChiffreAffaireRequest
 */
export interface ChiffreAffaireRequest {
    /**
     * @generated from protobuf field: optional string date_debut = 1;
     */
    dateDebut?: string;
    /**
     * @generated from protobuf field: optional string date_fin = 2;
     */
    dateFin?: string;
}
/**
 * @generated from protobuf message admin.ChiffreAffaireResponse
 */
export interface ChiffreAffaireResponse {
    /**
     * @generated from protobuf field: float valeur = 1;
     */
    valeur: number;
}
/**
 * @generated from protobuf message admin.GainRequest
 */
export interface GainRequest {
    /**
     * @generated from protobuf field: optional string date_debut = 1;
     */
    dateDebut?: string;
    /**
     * @generated from protobuf field: optional string date_fin = 2;
     */
    dateFin?: string;
}
/**
 * @generated from protobuf message admin.GainResponse
 */
export interface GainResponse {
    /**
     * @generated from protobuf field: float valeur = 1;
     */
    valeur: number;
}
/**
 * @generated from protobuf message admin.ImportBienRequest
 */
export interface ImportBienRequest {
    /**
     * @generated from protobuf field: bytes content = 1;
     */
    content: Uint8Array;
}
/**
 * @generated from protobuf message admin.ImportLocationRequest
 */
export interface ImportLocationRequest {
    /**
     * @generated from protobuf field: bytes content = 1;
     */
    content: Uint8Array;
}
/**
 * @generated from protobuf message admin.ImportCommissionRequest
 */
export interface ImportCommissionRequest {
    /**
     * @generated from protobuf field: bytes content = 1;
     */
    content: Uint8Array;
}
/**
 * @generated from protobuf message admin.NewLocationRequest
 */
export interface NewLocationRequest {
    /**
     * @generated from protobuf field: string bien_reference = 1;
     */
    bienReference: string;
    /**
     * @generated from protobuf field: string client = 2;
     */
    client: string;
    /**
     * @generated from protobuf field: string date_entree = 3;
     */
    dateEntree: string;
    /**
     * @generated from protobuf field: uint32 durree_mois = 4;
     */
    durreeMois: number;
}
/**
 * @generated from protobuf message admin.NewLocationResponse
 */
export interface NewLocationResponse {
    /**
     * @generated from protobuf field: string id = 1;
     */
    id: string;
    /**
     * @generated from protobuf field: string bien_reference = 2;
     */
    bienReference: string;
    /**
     * @generated from protobuf field: string client = 3;
     */
    client: string;
    /**
     * @generated from protobuf field: string date_entree = 4;
     */
    dateEntree: string;
    /**
     * @generated from protobuf field: string date_sortie = 5;
     */
    dateSortie: string;
}
/**
 * @generated from protobuf message admin.ListBienRequest
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
     * @generated from protobuf field: optional string nom = 3;
     */
    nom?: string;
}
/**
 * @generated from protobuf message admin.ListBienResponse
 */
export interface ListBienResponse {
    /**
     * @generated from protobuf field: repeated admin.ListBienResponse.Bien biens = 1;
     */
    biens: ListBienResponse_Bien[];
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
 * @generated from protobuf message admin.ListBienResponse.Bien
 */
export interface ListBienResponse_Bien {
    /**
     * @generated from protobuf field: string reference = 1;
     */
    reference: string;
    /**
     * @generated from protobuf field: string nom = 2;
     */
    nom: string;
    /**
     * @generated from protobuf field: string description = 3;
     */
    description: string;
    /**
     * @generated from protobuf field: float loyer = 4;
     */
    loyer: number; // string proprio = 5;
}
/**
 * @generated from protobuf message admin.ListClientRequest
 */
export interface ListClientRequest {
    /**
     * @generated from protobuf field: optional uint32 offset = 1;
     */
    offset?: number;
    /**
     * @generated from protobuf field: optional uint32 limit = 2;
     */
    limit?: number;
    /**
     * @generated from protobuf field: optional string nom = 3;
     */
    nom?: string;
    /**
     * @generated from protobuf field: optional string email = 4;
     */
    email?: string;
}
/**
 * @generated from protobuf message admin.ListClientResponse
 */
export interface ListClientResponse {
    /**
     * @generated from protobuf field: repeated admin.ListClientResponse.Client biens = 1;
     */
    biens: ListClientResponse_Client[];
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
 * @generated from protobuf message admin.ListClientResponse.Client
 */
export interface ListClientResponse_Client {
    /**
     * @generated from protobuf field: string email = 1;
     */
    email: string;
    /**
     * @generated from protobuf field: string nom = 2;
     */
    nom: string;
}
// @generated message type with reflection information, may provide speed optimized methods
class Empty$Type extends MessageType<Empty> {
    constructor() {
        super("admin.Empty", []);
    }
}
/**
 * @generated MessageType for protobuf message admin.Empty
 */
export const Empty = new Empty$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginRequest$Type extends MessageType<LoginRequest> {
    constructor() {
        super("admin.LoginRequest", [
            { no: 2, name: "password", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.LoginRequest
 */
export const LoginRequest = new LoginRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LoginResponse$Type extends MessageType<LoginResponse> {
    constructor() {
        super("admin.LoginResponse", [
            { no: 1, name: "session_token", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.LoginResponse
 */
export const LoginResponse = new LoginResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ChiffreAffaireRequest$Type extends MessageType<ChiffreAffaireRequest> {
    constructor() {
        super("admin.ChiffreAffaireRequest", [
            { no: 1, name: "date_debut", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "date_fin", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ChiffreAffaireRequest
 */
export const ChiffreAffaireRequest = new ChiffreAffaireRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ChiffreAffaireResponse$Type extends MessageType<ChiffreAffaireResponse> {
    constructor() {
        super("admin.ChiffreAffaireResponse", [
            { no: 1, name: "valeur", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ChiffreAffaireResponse
 */
export const ChiffreAffaireResponse = new ChiffreAffaireResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GainRequest$Type extends MessageType<GainRequest> {
    constructor() {
        super("admin.GainRequest", [
            { no: 1, name: "date_debut", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "date_fin", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.GainRequest
 */
export const GainRequest = new GainRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GainResponse$Type extends MessageType<GainResponse> {
    constructor() {
        super("admin.GainResponse", [
            { no: 1, name: "valeur", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.GainResponse
 */
export const GainResponse = new GainResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ImportBienRequest$Type extends MessageType<ImportBienRequest> {
    constructor() {
        super("admin.ImportBienRequest", [
            { no: 1, name: "content", kind: "scalar", T: 12 /*ScalarType.BYTES*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ImportBienRequest
 */
export const ImportBienRequest = new ImportBienRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ImportLocationRequest$Type extends MessageType<ImportLocationRequest> {
    constructor() {
        super("admin.ImportLocationRequest", [
            { no: 1, name: "content", kind: "scalar", T: 12 /*ScalarType.BYTES*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ImportLocationRequest
 */
export const ImportLocationRequest = new ImportLocationRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ImportCommissionRequest$Type extends MessageType<ImportCommissionRequest> {
    constructor() {
        super("admin.ImportCommissionRequest", [
            { no: 1, name: "content", kind: "scalar", T: 12 /*ScalarType.BYTES*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ImportCommissionRequest
 */
export const ImportCommissionRequest = new ImportCommissionRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class NewLocationRequest$Type extends MessageType<NewLocationRequest> {
    constructor() {
        super("admin.NewLocationRequest", [
            { no: 1, name: "bien_reference", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "client", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "date_entree", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "durree_mois", kind: "scalar", T: 13 /*ScalarType.UINT32*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.NewLocationRequest
 */
export const NewLocationRequest = new NewLocationRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class NewLocationResponse$Type extends MessageType<NewLocationResponse> {
    constructor() {
        super("admin.NewLocationResponse", [
            { no: 1, name: "id", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "bien_reference", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "client", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "date_entree", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 5, name: "date_sortie", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.NewLocationResponse
 */
export const NewLocationResponse = new NewLocationResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListBienRequest$Type extends MessageType<ListBienRequest> {
    constructor() {
        super("admin.ListBienRequest", [
            { no: 1, name: "offset", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 2, name: "limit", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "nom", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ListBienRequest
 */
export const ListBienRequest = new ListBienRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListBienResponse$Type extends MessageType<ListBienResponse> {
    constructor() {
        super("admin.ListBienResponse", [
            { no: 1, name: "biens", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => ListBienResponse_Bien },
            { no: 2, name: "offset", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "limit", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
            { no: 4, name: "total", kind: "scalar", T: 13 /*ScalarType.UINT32*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ListBienResponse
 */
export const ListBienResponse = new ListBienResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListBienResponse_Bien$Type extends MessageType<ListBienResponse_Bien> {
    constructor() {
        super("admin.ListBienResponse.Bien", [
            { no: 1, name: "reference", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "nom", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 3, name: "description", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "loyer", kind: "scalar", T: 2 /*ScalarType.FLOAT*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ListBienResponse.Bien
 */
export const ListBienResponse_Bien = new ListBienResponse_Bien$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListClientRequest$Type extends MessageType<ListClientRequest> {
    constructor() {
        super("admin.ListClientRequest", [
            { no: 1, name: "offset", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 2, name: "limit", kind: "scalar", opt: true, T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "nom", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ },
            { no: 4, name: "email", kind: "scalar", opt: true, T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ListClientRequest
 */
export const ListClientRequest = new ListClientRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListClientResponse$Type extends MessageType<ListClientResponse> {
    constructor() {
        super("admin.ListClientResponse", [
            { no: 1, name: "biens", kind: "message", repeat: 1 /*RepeatType.PACKED*/, T: () => ListClientResponse_Client },
            { no: 2, name: "offset", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
            { no: 3, name: "limit", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
            { no: 4, name: "total", kind: "scalar", T: 13 /*ScalarType.UINT32*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ListClientResponse
 */
export const ListClientResponse = new ListClientResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ListClientResponse_Client$Type extends MessageType<ListClientResponse_Client> {
    constructor() {
        super("admin.ListClientResponse.Client", [
            { no: 1, name: "email", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
            { no: 2, name: "nom", kind: "scalar", T: 9 /*ScalarType.STRING*/ }
        ]);
    }
}
/**
 * @generated MessageType for protobuf message admin.ListClientResponse.Client
 */
export const ListClientResponse_Client = new ListClientResponse_Client$Type();
/**
 * @generated ServiceType for protobuf service admin.Auth
 */
export const Auth = new ServiceType("admin.Auth", [
    { name: "Login", options: {}, I: LoginRequest, O: LoginResponse }
]);
/**
 * @generated ServiceType for protobuf service admin.Comptes
 */
export const Comptes = new ServiceType("admin.Comptes", [
    { name: "ChiffreAffaire", serverStreaming: true, clientStreaming: true, options: {}, I: ChiffreAffaireRequest, O: ChiffreAffaireResponse },
    { name: "Gain", serverStreaming: true, clientStreaming: true, options: {}, I: GainRequest, O: GainResponse }
]);
/**
 * @generated ServiceType for protobuf service admin.Imports
 */
export const Imports = new ServiceType("admin.Imports", [
    { name: "Bien", clientStreaming: true, options: {}, I: ImportBienRequest, O: Empty },
    { name: "Location", clientStreaming: true, options: {}, I: ImportLocationRequest, O: Empty },
    { name: "Commission", clientStreaming: true, options: {}, I: ImportCommissionRequest, O: Empty }
]);
/**
 * @generated ServiceType for protobuf service admin.Location
 */
export const Location = new ServiceType("admin.Location", [
    { name: "NewLocation", options: {}, I: NewLocationRequest, O: NewLocationResponse },
    { name: "ListBien", options: {}, I: ListBienRequest, O: ListBienResponse },
    { name: "ListClient", options: {}, I: ListClientRequest, O: ListClientResponse }
]);
/**
 * @generated ServiceType for protobuf service admin.Database
 */
export const Database = new ServiceType("admin.Database", [
    { name: "Reset", options: {}, I: Empty, O: Empty }
]);
