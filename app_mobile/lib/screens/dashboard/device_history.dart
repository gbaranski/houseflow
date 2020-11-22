import 'package:cloud_firestore/cloud_firestore.dart';
import 'package:flutter/material.dart';
import 'package:houseflow/models/device.dart';
import 'package:houseflow/screens/history/single_history_view.dart';
import 'package:infinite_scroll_pagination/infinite_scroll_pagination.dart';

class DeviceHistoryList extends StatefulWidget {
  final PagingController<int, DocumentSnapshot> pagingController;

  final Future<QuerySnapshot> Function([DocumentSnapshot lastDocument])
      getDeviceHistory;

  const DeviceHistoryList(
      {Key key,
      @required this.getDeviceHistory,
      @required this.pagingController})
      : super(key: key);

  @override
  _DeviceHistoryListState createState() => _DeviceHistoryListState();
}

class _DeviceHistoryListState extends State<DeviceHistoryList> {
  @override
  void initState() {
    super.initState();
    widget.pagingController.addPageRequestListener((pageKey) {
      updateDeviceHistory(pageKey);
    });
  }

  Future<void> updateDeviceHistory(int pageKey) async {
    try {
      final lastDocument = widget.pagingController.itemList == null
          ? null
          : widget.pagingController.itemList[pageKey - 1];

      print(
          "Fetching device history i: $pageKey, last visible doc ${lastDocument?.id}");
      QuerySnapshot snapshot;
      if (lastDocument != null) {
        snapshot = await widget.getDeviceHistory(lastDocument);
      } else
        snapshot = await widget.getDeviceHistory();
      widget.pagingController
          .appendPage(snapshot.docs, pageKey + snapshot.docs.length);
    } catch (e) {
      print("Error occured when fetching device history $e");
      widget.pagingController.error = e;
    }
  }

  @override
  Widget build(BuildContext context) {
    return PagedSliverList<int, DocumentSnapshot>(
      key: Key('deviceHistoryList'),
      pagingController: widget.pagingController,
      builderDelegate: PagedChildBuilderDelegate<DocumentSnapshot>(
          itemBuilder: (context, item, index) => SingleDeviceHistory(
                deviceRequest: DeviceHistory.fromMap(item.data(), item.id),
              )),
    );
  }

  @override
  void dispose() {
    widget.pagingController.dispose();
    super.dispose();
  }
}
