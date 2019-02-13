import QtQuick 2.9
import QtQuick.Window 2.2

Window {
    id: window
    visible: true
    width: 640
    height: 480
    title: qsTr("Hello World")

    Flow {
        id: element1
        x: 0
        y: 0
        width: 640
        height: 480
        anchors.verticalCenter: parent.verticalCenter
        layoutDirection: Qt.LeftToRight
        flow: Flow.LeftToRight

        Text {
            id: element
            width: 111
            height: 97
            text: qsTr("boark boark worfdfffffffffffffffffffffffffff")
            font.pixelSize: 12
        }
    }
}

/*##^## Designer {
    D{i:1;anchors_width:640;anchors_x:0}
}
 ##^##*/
