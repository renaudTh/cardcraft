import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GameTableComponent } from './game-table.component';

describe('GameTableComponent', () => {
  let component: GameTableComponent;
  let fixture: ComponentFixture<GameTableComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [GameTableComponent]
    });
    fixture = TestBed.createComponent(GameTableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
