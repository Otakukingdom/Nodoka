//
// Created by mistlight on 3/5/2017.
//

#ifndef NODOKA_ABSTRACTELEMENTSCANNER_H
#define NODOKA_ABSTRACTELEMENTSCANNER_H

#include <QDir>
#include <QFile>
#include <QString>
#include <memory>

class AbstractElementScanner {

public:

    virtual void accept(const std::shared_ptr<QDir>&) = 0;
    virtual void accept(const std::shared_ptr<QFile>&) = 0;
    virtual void accept(QString directory) = 0;

};


#endif //NODOKA_ABSTRACTELEMENTSCANNER_H
