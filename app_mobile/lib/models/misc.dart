class ServerResponse {
  final String requestType;
  final List data;

  ServerResponse(this.requestType, this.data);

  ServerResponse.fromJson(Map<String, dynamic> json)
      : requestType = json['requestType'],
        data = json['data'];
}
