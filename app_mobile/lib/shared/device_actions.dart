import 'dart:math';

import 'package:flutter/material.dart';
import 'package:houseflow/shared/constants.dart';

class DeviceAction {
  final Function onSubmit;
  final String actionText;
  final Widget icon;

  DeviceAction(
      {@required this.onSubmit,
      @required this.actionText,
      @required this.icon});
}

class DeviceActions extends StatefulWidget {
  final List<DeviceAction> deviceActions;

  DeviceActions({@required this.deviceActions});

  @override
  _DeviceActionsState createState() => _DeviceActionsState();
}

class _DeviceActionsState extends State<DeviceActions>
    with SingleTickerProviderStateMixin {
  DeviceAction confirmation;

  void switchConfirmation(DeviceAction deviceAction) {
    setState(() {
      confirmation = deviceAction;
    });
  }

  Widget getDeviceActions() {
    return ButtonBar(
        layoutBehavior: ButtonBarLayoutBehavior.constrained,
        alignment: MainAxisAlignment.spaceEvenly,
        key: Key("DeviceActions"),
        children: widget.deviceActions
            .map((deviceAction) => IconButton(
                tooltip: "Mix water",
                icon: deviceAction.icon,
                onPressed: () => switchConfirmation(deviceAction)))
            .toList());
  }

  Widget needConfirmation() {
    return ButtonBar(
        key: Key("ConfirmationButtons"),
        layoutBehavior: ButtonBarLayoutBehavior.constrained,
        alignment: MainAxisAlignment.spaceEvenly,
        children: [
          IconButton(
              icon: Icon(
                Icons.done,
                color: Colors.lightGreen,
                size: ACTION_ICON_SIZE,
              ),
              onPressed: () => switchConfirmation(null)),
          IconButton(
              icon: Icon(
                Icons.close,
                color: Colors.redAccent,
                size: ACTION_ICON_SIZE,
              ),
              onPressed: () => switchConfirmation(null))
        ]);
  }

  @override
  Widget build(BuildContext context) {
    final visibleWidget =
        confirmation == null ? getDeviceActions() : needConfirmation();
    return Column(children: [
      Divider(
        indent: 30,
        endIndent: 30,
        thickness: 1,
      ),
      AnimatedSwitcher(
        duration: const Duration(milliseconds: 250),
        transitionBuilder: (Widget child, Animation<double> animation) {
          return ScaleTransition(
              scale: animation,
              child: FadeTransition(
                opacity: animation,
                child: child,
              ));
        },
        child: visibleWidget,
      )
    ]);
  }
}
