//
// Created by mistlight on 1/8/17.
//

#ifndef NODOKANATIVE_MEDIAPROPERTY_H
#define NODOKANATIVE_MEDIAPROPERTY_H


class MediaProperty {

private:
    long long duration;
    bool isNull;


public:
    // null constructor
    MediaProperty();

    // construct an object
    MediaProperty(long long mediaDuration);

    // check if the object is a null object
    bool isNullObject();

    long long getDuration();
};


#endif //NODOKANATIVE_MEDIAPROPERTY_H
