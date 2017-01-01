//
// Created by mistlight on 12/31/2016.
//

#include "SettingsForm.h"

SettingsForm::SettingsForm(QWidget *parent) :
  QWidget(parent), ui(new Ui::SettingsForm()) {
}

SettingsForm::~SettingsForm() {
    delete ui;
}
