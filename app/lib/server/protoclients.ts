import { ADMIN_BACK_URL, CLIENT_BACK_URL, PROPRIO_BACK_URL } from '$env/static/private';
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';

export const adminClient = new GrpcWebFetchTransport({
	baseUrl: ADMIN_BACK_URL
});

export const clientClient = new GrpcWebFetchTransport({
	baseUrl: CLIENT_BACK_URL
});

export const proprioClient = new GrpcWebFetchTransport({
	baseUrl: PROPRIO_BACK_URL
});
